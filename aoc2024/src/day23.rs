use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let graph = Graph::new(input);
    Ok(Solution::default()
        .part1(part1(&graph))
        .part2(part2(&graph)))
}

const MAX: usize = 26 * 26;

struct Graph<'a> {
    nodes: Vec<&'a str>,
    edges: Box<[[bool; MAX]; MAX]>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Self {
        let mut counter = 0;
        let mut id = [usize::MAX; MAX];
        let mut nodes = Vec::new();
        let mut edges = Box::new([[false; MAX]; MAX]);
        let key = |node: &str| {
            let name = node.as_bytes();
            (name[0] - b'a') as usize * 26 + (name[1] - b'a') as usize
        };
        for line in input.lines() {
            let (a, b) = line
                .split_once('-')
                .expect("There should always be 2 nodes");
            if id[key(a)] == usize::MAX {
                id[key(a)] = counter;
                nodes.push(a);
                counter += 1;
            }
            if id[key(b)] == usize::MAX {
                id[key(b)] = counter;
                nodes.push(b);
                counter += 1;
            }
            edges[id[key(a)]][id[key(b)]] = true;
            edges[id[key(b)]][id[key(a)]] = true;
        }
        Self { nodes, edges }
    }
}

#[must_use]
fn part1(graph: &Graph) -> u32 {
    let mut total = 0;
    let len = graph.nodes.len();
    // For each pair of nodes (a, b)
    for a in 0..len {
        for b in a + 1..len {
            // If the nodes are connected, try to find a 3rd node connected to both of them
            if graph.edges[a][b] {
                // For each 3rd node
                for c in b + 1..len {
                    let initial = |i: usize| graph.nodes[i].as_bytes()[0];
                    // If it's connected to a and b, and any of the nodes' names start with a `t`,
                    // increment the amount of valid cliques found
                    total += u32::from(
                        (graph.edges[c][a] && graph.edges[c][b])
                            && [initial(a), initial(b), initial(c)].contains(&b't'),
                    );
                }
            }
        }
    }
    total
}

#[must_use]
fn part2(graph: &Graph) -> String {
    let mut front: Vec<Vec<usize>> = (0..graph.nodes.len()).map(|i| vec![i]).collect();
    let mut back = Vec::with_capacity(front.len());
    // While we have cliques of size N, calculate all possible cliques of size N+1
    while !front.is_empty() {
        // Reset the buffer for cliques of size N+1
        back.clear();
        // For each clique of size N
        for clique in &front {
            // Get its last node ID
            let last = clique
                .last()
                .expect("All cliques should have at least 1 element");
            // For each node after the last node in the current clique. This ensures we don't
            // try to create repeated cliques whose only difference is the order of the nodes
            for new_node in *last..graph.nodes.len() {
                // If the new node can extend the current clique to size N+1, store the new clique
                if clique.iter().all(|&node| graph.edges[new_node][node]) {
                    back.push(
                        clique
                            .iter()
                            .copied()
                            .chain(std::iter::once(new_node))
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }
        // Swap the buffers to reuse them
        std::mem::swap(&mut front, &mut back);
    }
    // We have no more cliques of size N => biggest clique has size N-1 and is in the back buffer
    assert_eq!(back.len(), 1);
    // Get the biggest clique
    let max_clique = back.first().expect("There should be a maximum clique");
    // Transform the result to the requested format
    max_clique
        .iter()
        // Convert the node IDs to their names
        .map(|&x| graph.nodes[x])
        // Sort by the names alphabetically
        .sorted()
        // Join the names using `,` as separator into a string
        .join(",")
}
