use std::{
    cmp::{Ord, PartialOrd},
    fmt::Display,
    ops::Add,
};

pub fn pop_front_min_heap<T: PartialOrd + Copy + Ord>(x: &mut [T]) -> Option<T> {
    if x.is_empty() {
        return None;
    }

    if x.len() == 1 {
        return Some(x[0]);
    }

    let ret = x[0];

    // swap last node and root node
    let last = x[x.len() - 1];
    x[0] = last;

    let mut idx = 0;

    loop {
        let root_node = x[idx];

        // idx points to root node

        // get child nodes
        let c1 = x.get(2 * idx + 1);
        let c2 = x.get(2 * idx + 2);

        match (c1, c2) {
            (Some(&left), Some(&right)) => {
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
            (Some(&left), None) => {
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
pub fn build_min_heap<T: PartialOrd + Copy>(x: &mut [T]) {
    for mut idx in 0..x.len() {
        let c_node = x[idx];

        loop {
            if idx == 0 {
                break;
            }
            // get value of sparent node
            let p_node = x[(idx - 1) / 2];
            if p_node > c_node {
                // swap current node with parent node
                x.swap(idx, (idx - 1) / 2);
                // set index to parent node
                idx = (idx - 1) / 2;
            } else {
                // else no swapping required
                break;
            }
        }
    }
}

// function needed for merging nodes

// merge nodes into new binary tree (complete BT for now)
pub fn merge_trees<T: Copy + Add<Output = T>>(
    // this vec can be empty
    x1: Vec<Option<T>>,
    // this can cannot be empty
    x2: Vec<Option<T>>,
) -> Vec<Option<T>> {
    // can be empty, or have just one node
    if x1.is_empty() {
        return x2;
    }

    assert!(!x1.is_empty() && !x2.is_empty());

    // has to have at least 2 nodes if not empty

    let mut v = vec![];

    v.push(Some(x1[0].unwrap() + x2[0].unwrap()));

    let mut idx = 0;
    let mut tree_width = 1;

    // TODO: reserve complete capacity upfront, and use .fill() + .set_len()
    loop {
        match (x1.get(idx..idx + tree_width), x2.get(idx..idx + tree_width)) {
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

pub fn print_bt<T: Display + Copy>(x: &[Option<T>]) {
    for &x in x {
        if let Some(x) = x {
            print!("{x}, ");
        } else {
            print!("[X], ");
        }
    }
    println!();
}

fn main() {
    // sorted frequency map
    // a, b, c, d, e, ... etc
    let mut freqs = [10, 6, 5, 1, 3];

    let mut freq_ptr = &mut freqs[..];

    build_min_heap(&mut freq_ptr);
    println!("{freq_ptr:?}");

    // TODO use simple stack
    let mut two_nodes = vec![];

    let mut huffman_tree = vec![];

    while let Some(node) = pop_front_min_heap(&mut freq_ptr) {
        {
            let len = freq_ptr.len();
            freq_ptr = &mut freq_ptr[..len - 1];
        }

        two_nodes.push(node);

        println!("{freq_ptr:?}");
        // dbg!(&two_nodes);

        if two_nodes.len() == 2 {
            dbg!(&two_nodes);
            let new_node = vec![
                Some(two_nodes[0] + two_nodes[1]),
                Some(two_nodes[0]),
                Some(two_nodes[1]),
            ];

            huffman_tree = merge_trees(huffman_tree, new_node);
            two_nodes.clear();
        }

        // check if two_nodes length is 1 or 2 at the end
    }
    dbg!(&two_nodes);

    // add in root node
    if let Some(&last_node) = two_nodes.get(0) {
        huffman_tree = merge_trees(huffman_tree, vec![Some(last_node)]);
    }

    print_bt(&huffman_tree);

    // let mut y = &mut freqs[..];
    // while let Some(x) = pop_front_min_heap(y) {
    //     dbg!(x);

    //     let len = y.len();
    //     y = &mut y[..len - 1];
    // }

    // println!("{freqs:?}");

    // TODO fix really bad performance
    // let new_vec = freqs.iter().map(|x| vec![*x]).collect::<Vec<_>>();

    // let mut min_heap = vec![];

    // let x1 = vec![6, 2, 4];
    // let x1 = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
    // let x2 = vec![9, 5, 4];

    // {
    //     // let x1 = x1.iter().copied().map(Some).collect();
    //     let x2 = x2.iter().copied().map(Some).collect();
    //     let x3 = merge_trees(x1, x2);
    //     print_bt(&x3);
    // }
}
