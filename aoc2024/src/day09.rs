use std::error::Error;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    Ok(Solution::default()
        .part1(part1(input.as_bytes()))
        .part2(part2(input.as_bytes())))
}

const fn range_sum(a: usize, b: usize) -> usize {
    ((b - a) * (a + b - 1)) / 2
}

#[must_use]
fn part1(input: &[u8]) -> usize {
    let mut checksum = 0;
    let get = |i| usize::from(input[i] - b'0');
    // Start pointer information
    let mut start = 1;
    let mut id_start = 1;
    let mut space = get(start);
    // End pointer information
    let mut end = input.len() - 1;
    end -= usize::from(end % 2 == 1);
    let mut id_end = input.len() / 2 - 1;
    let mut data = get(end);
    // Address of the first free space
    let mut addr = get(0);
    macro_rules! advance_start {
        ($increment_checksum:literal) => {
            // Move the start pointer
            start += 2;
            // If the pointers cross, we have finished
            if start > end {
                // Update the checksum if there is a remaining data element
                if $increment_checksum && data != 0 {
                    checksum += id_end * range_sum(addr, addr + data);
                }
                break;
            }
            // Read the next free space
            space = get(start);
            // Consume the next data element from the start
            let mid = get(start - 1);
            checksum += id_start * range_sum(addr, addr + mid);
            addr += mid; // Increment the address of the first gap
            id_start += 1; // Calculate the ID of the next data element from the start
        };
    }
    // Repeat until the pointers have crossed
    while start < end {
        // If the current free space is big enough, move the remaining data element
        if space >= data {
            checksum += id_end * range_sum(addr, addr + data); // Increment the value of the checksum
            space -= data; // Reduce the remaining free space
            addr += data; // Increment the address of the first gap
            end -= 2; // Move the end pointer to the next data element
            data = get(end); // Read the next data element
            id_end -= 1; // Calculate the ID of the next data element from the end

            // If we don't have any remaining free space, consume the next data element from the
            // start and move the start pointer
            if space == 0 {
                advance_start!(false);
            }
        // If the current free space isn't big enough, use what's remaining of it and get the next
        // one
        } else {
            checksum += id_end * range_sum(addr, addr + space);
            data -= space;
            addr += space;
            advance_start!(true);
        }
    }
    checksum
}

#[must_use]
fn part2(input: &[u8]) -> usize {
    let mut checksum = 0;
    let get = |i| input[i] - b'0';
    // Free space remaining on each gap
    let mut free_space = input.to_vec();
    // End pointer information
    let mut end = input.len() - 1;
    end -= usize::from(end % 2 == 1);
    let mut id_end = input.len() / 2 - 1;
    // While there are data elements to process
    while end > 0 {
        // Get the current data element
        let data = get(end);
        // Search for the first gap big enough for the data element
        let mut start = 1;
        let mut addr = usize::from(get(0));
        loop {
            // If we didn't find any gaps big enough for this element, select its current address
            if start > end {
                addr -= usize::from(data);
                break;
            }
            // If this gap is big enough, select the address of this gap
            if free_space[start] - b'0' >= data {
                // Update the address taking into account the space of this gap that's already been
                // used
                addr += usize::from(get(start) - (free_space[start] - b'0'));
                // Reduce the remaining free space on this gap
                free_space[start] -= data;
                break;
            }
            // The gap wasn't big enough: skip it and update the address considering the size of
            // this gap and the next data element from the start
            addr += usize::from(get(start) + get(start + 1));
            start += 2;
        }
        // Update the checksum considering the data element is stored in the address selected
        checksum += id_end * range_sum(addr, addr + usize::from(data));
        // Continue with the next data element from the end
        id_end -= 1;
        end -= 2;
    }
    checksum
}
