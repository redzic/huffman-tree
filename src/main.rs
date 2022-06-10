use std::{
    cmp::{Ord, PartialOrd},
    collections::HashMap,
    fmt::Debug,
    num::NonZeroUsize,
};

use bitvec::prelude::*;

pub fn pop_front_min_heap(x: &mut [BinaryHeap]) -> Option<BinaryHeap> {
    if x.is_empty() {
        return None;
    }

    let ret = x[0].clone();

    // swap last node and root node
    let last = x[x.len() - 1].clone();
    x[0] = last;

    let mut idx = 0;

    loop {
        let root_node = &x[idx];

        // idx points to root node

        // get child nodes
        let c1 = x.get(2 * idx + 1);
        let c2 = x.get(2 * idx + 2);

        match (c1, c2) {
            (Some(left), Some(right)) => {
                let min_child_idx = if left <= right { 1 } else { 2 };

                let min = *[left, right, root_node].iter().min().unwrap();
                if root_node != min {
                    // need to swap with largest in that case
                    // and traverse down the tree
                    x.swap(idx, 2 * idx + min_child_idx);

                    // adjust index, go to child node that was larger
                    idx = 2 * idx + min_child_idx;
                } else {
                    break;
                }
            }
            (Some(left), None) => {
                // compare last child node with parent node
                // since this is a min heap, the parent node should always be less than
                // or equal to its children
                if left < root_node {
                    x.swap(idx, 2 * idx + 1);
                }
                break;
            }
            _ => break,
        }
    }

    Some(ret)
}

// build min heap in-place
pub fn build_min_heap(x: &mut [BinaryHeap]) {
    for idx in 0..x.len() {
        move_node_min_heap(idx, x);
    }
}

pub fn move_node_min_heap(mut node_idx: usize, tree: &mut [BinaryHeap]) {
    let node = tree[node_idx].tree[0];
    while node_idx != 0 {
        // get value of parent node
        let p_node = tree[(node_idx - 1) / 2].tree[0];
        if p_node > node {
            // swap current node with parent node
            tree.swap(node_idx, (node_idx - 1) / 2);
            // set index to parent node
            node_idx = (node_idx - 1) / 2;
        } else {
            // else no swapping required
            break;
        }
    }
}

// function needed for merging nodes

pub fn merge_trees(
    // this vec can be empty
    tree: Vec<Option<NonZeroUsize>>,
    // this can cannot be empty
    nnode: Vec<Option<NonZeroUsize>>,
) -> Vec<Option<NonZeroUsize>> {
    // can be empty, or have just one node
    if tree.is_empty() {
        return nnode;
    }

    debug_assert!(!tree.is_empty() && !nnode.is_empty());

    // has to have at least 2 nodes if not empty

    // TODO check if there's a better way to do this

    // check if the root node is equal to the left child node

    if !tree.is_empty() && nnode.len() >= 3 && tree[0].is_some() {
        // TODO unfuck code
        if tree[0].unwrap() == nnode[1].unwrap() || tree[0].unwrap() == nnode[2].unwrap() {
            let mut ntree = vec![nnode[0], tree[0]];

            if tree[0].unwrap() == nnode[1].unwrap() {
                ntree.push(nnode[2]);
            } else {
                ntree.push(nnode[1]);
            }

            // left side of tree is copied, right side is none
            let mut tree_width = 2;
            let mut idx = 1;
            while let Some(row) = tree.get(idx..idx + tree_width) {
                ntree.extend(row);
                ntree.extend(std::iter::repeat(None).take(tree_width));

                idx += tree_width;
                tree_width *= 2;
            }

            return ntree;
        }
    }

    let mut v = unsafe {
        vec![Some(NonZeroUsize::new_unchecked(
            tree[0].unwrap().get() + nnode[0].unwrap().get(),
        ))]
    };

    let mut idx = 0;
    let mut tree_width = 1;

    // TODO: reserve complete capacity upfront, and use .fill() + .set_len()
    loop {
        match (
            tree.get(idx..idx + tree_width),
            nnode.get(idx..idx + tree_width),
        ) {
            (None, None) => break,
            (l, r) => {
                for x in [l, r] {
                    v.reserve(2 * tree_width);
                    if let Some(x) = x {
                        v.extend(x.iter().copied());
                    } else {
                        v.extend(std::iter::repeat(None).take(tree_width));
                    }
                }
            }
        }

        idx += tree_width;
        tree_width *= 2;
    }

    v
}

pub fn print_bt(x: &[Option<NonZeroUsize>]) {
    for &x in x {
        if let Some(x) = x {
            print!("{x}, ");
        } else {
            print!("[X], ");
        }
    }
    println!();
}

#[macro_export]
macro_rules! create_freqs {
    ($($x:expr),* $(,)?) => {
        [
        $(
            BinaryHeap::root($x),
        )*
        ]
    };
}

#[derive(Debug, Clone)]
pub struct BinaryHeap {
    tree: Vec<Option<NonZeroUsize>>,
}

impl BinaryHeap {
    fn root(x: usize) -> Self {
        Self {
            tree: vec![NonZeroUsize::new(x)],
        }
    }
}

impl PartialOrd for BinaryHeap {
    // compare first element
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tree[0].partial_cmp(&other.tree[0])
    }
}

impl PartialEq for BinaryHeap {
    fn eq(&self, other: &Self) -> bool {
        self.tree[0] == other.tree[0]
    }
}

impl Eq for BinaryHeap {}

impl Ord for BinaryHeap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tree[0].cmp(&other.tree[0])
    }
}

#[derive(Copy, Clone)]
pub struct HuffmanCode {
    // max code length is 32 bits
    code: u32,
    num_bits: u32,
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: ./heap-rs <file>");
        std::process::exit(1);
    }

    // TODO check how to display a more user-facing error message
    let string = std::fs::read_to_string(&args[1])?;

    // frequency map initialized to 0 for all characters
    // (character, frequency)
    let mut freq_map = vec![(0u8, 0usize); 256];

    for (idx, (x, _)) in freq_map.iter_mut().enumerate() {
        *x = idx as u8;
    }

    for c in string.as_bytes() {
        freq_map[*c as usize].1 += 1;
    }

    freq_map.retain(|(_, freq)| *freq != 0);

    let mut freqs: Vec<BinaryHeap> = freq_map
        .iter()
        .map(|(_, freq)| BinaryHeap::root(*freq))
        .collect();

    // frequency map
    // each element is a binary tree
    // let mut freqs = create_freqs![1, 2, 3, 4, 5, 6, 999];

    build_min_heap(&mut freqs);

    let mut idx = freqs.len();

    while idx != 1 {
        let node1 = pop_front_min_heap(&mut freqs[..idx]).unwrap();
        idx -= 1;
        let node2 = pop_front_min_heap(&mut freqs[..idx]).unwrap();
        idx -= 1;

        freqs[idx] = BinaryHeap {
            tree: merge_trees(node1.tree, node2.tree),
        };
        move_node_min_heap(idx, &mut freqs);

        idx += 1;
    }

    fn code_length(x: usize) -> u32 {
        (usize::BITS - ((x + 1).leading_zeros())) - 1
    }

    let mut idx = 0;
    let mut tree_width = 1;

    // TODO maybe can just use simple lookup table for this
    let mut huffman_table: HashMap<u8, HuffmanCode> = HashMap::new();

    // TODO replace with less bad solution
    while idx + tree_width <= freqs[0].tree.len() {
        for i in 0..tree_width {
            // check if current node is a leaf node,
            // based on whether a child node exists and has a value.
            let leaf_node = if let Some(Some(node)) = freqs[0].tree.get(idx + i) {
                match freqs[0].tree.get(2 * (idx + i) + 1) {
                    None | Some(&None) => Some(*node),
                    _ => None,
                }
            } else {
                None
            };
            if let Some(leaf_node) = leaf_node {
                // get length of code
                let length = code_length(idx + i);
                let mut period_log2 = length - 1;
                let mut code = 0;

                // find character with the same frequency as leaf_node
                let removal_index = freq_map
                    .iter()
                    .position(|(_k, v)| *v == leaf_node.get())
                    .unwrap();
                let (symbol, _) = freq_map.remove(removal_index);

                for _ in 0..length {
                    code = code << 1 | (i as u32 >> period_log2) & 1;
                    // result of subtraction from 0 is never read afterwards
                    period_log2 = period_log2.wrapping_sub(1);
                }
                huffman_table.insert(
                    symbol,
                    HuffmanCode {
                        code,
                        num_bits: length,
                    },
                );
            }
        }

        idx += tree_width;
        tree_width *= 2;
    }

    // code the final thing

    let mut output_vec: BitVec<usize, Lsb0> = BitVec::default();
    for c in string.as_bytes() {
        // lookup code from table
        let huffman_code = huffman_table[c];
        for i in 0..huffman_code.num_bits {
            // TODO: perf-wise, this is probably extremely bad
            output_vec.push((huffman_code.code >> i) & 1 != 0);
        }
    }

    dbg!(output_vec.len() / 8);

    Ok(())
}
