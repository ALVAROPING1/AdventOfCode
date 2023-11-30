fn main() {
    let start = std::time::Instant::now();

    match aoc2023::current::run() {
        Ok(solution) => {
            let elapsed = start.elapsed();
            print!("{solution}");
            println!("Elapsed: {elapsed:?}");
        }
        Err(err) => eprintln!("{err}"),
    }
}
