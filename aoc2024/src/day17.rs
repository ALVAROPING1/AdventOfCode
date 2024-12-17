use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let mut computer = Computer::new(input);
    Ok(Solution::default()
        .part1(part1(&mut computer))
        .part2(part2(&mut computer)))
}

struct Computer<'a> {
    pc: usize,
    registers: [u64; 3],
    program: &'a [u8],
}

impl<'a> Computer<'a> {
    fn new(input: &'a str) -> Self {
        let (state, program) = input
            .split_once("\n\n")
            .expect("There should always be 2 sections");
        let mut registers = [0; 3];
        for (i, line) in state.lines().enumerate() {
            registers[i] = line[12..]
                .parse()
                .expect("All register values should be valid");
        }
        Self {
            pc: 0,
            registers,
            program: program[9..program.len() - 1].as_bytes(),
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0, 0, 0];
    }

    fn fetch(&mut self) -> (u8, u8) {
        let get = |i| self.program[i * 2] - b'0';
        let instruction = (get(self.pc), get(self.pc + 1));
        self.pc += 2;
        instruction
    }

    const fn is_running(&self) -> bool {
        self.pc * 2 < self.program.len()
    }

    fn combo_arg(&self, arg: u8) -> u64 {
        match arg {
            0..=3 => arg.into(),
            4..=6 => self.registers[usize::from(arg) - 4],
            _ => unreachable!("Combo arguments only use values [0, 6]"),
        }
    }

    fn execute_instruction(&mut self, stdout: &mut String) {
        let (opcode, arg) = self.fetch();
        let dv = |arg| {
            self.registers[0]
                .checked_shr(self.combo_arg(arg).try_into().unwrap_or(u32::MAX))
                .unwrap_or(0)
        };
        match opcode {
            0 /* adv */ => self.registers[0] = dv(arg),
            1 /* bxl */ => self.registers[1] ^= u64::from(arg),
            2 /* bst */ => self.registers[1] = self.combo_arg(arg) % 8,
            3 /* jnz */ if self.registers[0] != 0 => self.pc = usize::from(arg),
            4 /* bxc */ => self.registers[1] ^= self.registers[2],
            5 /* out */ => {
                let val = self.combo_arg(arg) % 8;
                let val = u8::try_from(val).expect("The value should always fit");
                stdout.push((val + b'0') as char);
                stdout.push(',');
            }
            6 /* bdv */ => self.registers[1] = dv(arg),
            7 /* cdv */ => self.registers[2] = dv(arg),
            _ => {}
        }
    }

    fn run_program(&mut self, stdout: &mut String) {
        while self.is_running() {
            self.execute_instruction(stdout);
        }
        stdout.pop();
    }
}

#[must_use]
fn part1(computer: &mut Computer) -> String {
    let mut stdout = String::new();
    computer.run_program(&mut stdout);
    stdout
}

// Each iteration of the program, it prints a value `f(A)` then divides A by 8. The program ends
// when A is 0. This means that the sequence of printed values will be `..., f(X * 8² + Y * 8 + Z), f(X * 8 + Y), f(X)`,
// where the variables have to be 3-bit numbers. We can take advantage of this by building the
// answer from the end: we find the value of X that makes the last printed value correct, then the
// value of Y that makes the previous one correct, and so on. At each step, there are 8
// possibilities to make the currently considered printed value correct, and any of them that don't
// make the printed value correct can't lead to the correct initial value of A (since the full
// printed sequence will difer at that position)
fn dfs(computer: &mut Computer, stdout: &mut String, a: u64, pos: usize) -> Option<u64> {
    for i in 0..8 {
        // Append the current 3-bit number to the end of the result
        let a = (a << 3) | i;
        // Execute the program
        stdout.clear();
        computer.reset();
        computer.registers[0] = a;
        computer.run_program(stdout);
        // If the first result doesn't match the currently considered position in the sequence, this path
        // is incorrect. All the following results will always match the sequence
        if stdout.as_bytes()[0] != computer.program[pos] {
            continue;
        }
        // The sequence is correct from the current position to the end
        // If the current position is the start of the sequence, we have found the answer
        if pos == 0 {
            return Some(a);
        }
        // Otherwise, keep building the answer recursively
        if let Some(a) = dfs(computer, stdout, a, pos - 2) {
            return Some(a);
        }
    }
    None // We didn't find an answer through this path
}

#[must_use]
fn part2(computer: &mut Computer) -> u64 {
    let mut stdout = String::new();
    dfs(computer, &mut stdout, 0, computer.program.len() - 1).expect("No answer found")
}
