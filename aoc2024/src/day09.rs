use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    let get = |i| usize::from(input[i] - b'0');
    // Map of gap size to min-heaps of gaps with that size, ordered by the address of the gaps
    let mut free_space = vec![BinaryHeap::new(); 10];
    // Data elements to process, stored as tuples (address, size). Their ID is their index in the
    // vector
    let mut data_elements = Vec::with_capacity(input.len() / 2 - 1);
    // Current address
    let mut addr = get(0);
    // For each pair of (gap, data_element)
    for i in (1..input.len() - 1).step_by(2) {
        // Get the size of the gap
        let space = get(i);
        // Store it in its corresponding heap
        free_space[space].push(Reverse(addr));
        // Increment the current address by the size of the gap
        addr += space;
        // Get the size of the data element
        let data = get(i + 1);
        // Record it along with its address
        data_elements.push((addr, data));
        // Increment the current address by the size of the data element
        addr += data;
    }
    // For each data element starting from the end
    for (id, (data_addr, size)) in data_elements.iter().enumerate().rev() {
        // Calculate its ID
        let id = id + 1;
        // Find the first gap big enough for the data element
        let min = (*size..10)
            .filter_map(|space_size| Some((space_size, free_space[space_size].peek()?.0)))
            .min_by_key(|x| x.1);
        // If a gap was found
        if let Some((space_size, addr)) = min {
            // If the address of the gap is before the address the data element is already at
            if addr < *data_addr {
                // Pop the gap from its heap
                let Reverse(addr) = free_space[space_size]
                    .pop()
                    .expect("We already checked that this heap isn't empty");
                // Update the checksum with the contribution of this data element
                checksum += id * range_sum(addr, addr + *size);
                // Calculate the remaining space and starting address of this gap
                let space_size = space_size - size;
                let addr = addr + *size;
                // Add the updated gap back to its corresponding heap
                free_space[space_size].push(Reverse(addr));
                // Continue with the next data element
                continue;
            }
        }
        // If no gap was found or it was after the address of the data element, update the checksum
        // with the contribution of this data element without moving it
        checksum += id * range_sum(*data_addr, *data_addr + *size);
    }
    checksum
}
