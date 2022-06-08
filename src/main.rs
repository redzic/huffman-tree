use std::{
    cmp::{Ord, PartialOrd},
    fmt::{Debug, Display},
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
    for idx in 0..x.len() {
        move_node_min_heap(idx, x);
    }
}

pub fn move_node_min_heap<T: Copy + PartialOrd>(mut node_idx: usize, tree: &mut [T]) {
    let node = tree[node_idx];

    while node_idx != 0 {
        // get value of parent node
        let p_node = tree[(node_idx - 1) / 2];
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

// merge nodes into new binary tree (complete BT for now)
pub fn merge_trees<T: Copy + Debug + Display + PartialEq + Add<Output = T>>(
    // this vec can be empty
    tree: Vec<Option<T>>,
    // this can cannot be empty
    nnode: Vec<Option<T>>,
) -> Vec<Option<T>> {
    // can be empty, or have just one node
    if tree.is_empty() {
        return nnode;
    }

    assert!(!tree.is_empty() && !nnode.is_empty());

    println!("\nRECEIVED TREE:");
    print_bt(&tree);
    println!("RECEIVED NEW NODE:");
    print_bt(&nnode);
    println!();

    // has to have at least 2 nodes if not empty

    // TODO check if there's a better way to do this

    // check if the root node is equal to the left child node

    if !tree.is_empty() && nnode.len() >= 3 && tree[0].is_some() {
        // TODO unfuck code
        if tree[0].unwrap() == nnode[1].unwrap() || tree[0].unwrap() == nnode[2].unwrap() {
            let mut ntree = vec![];

            ntree.push(nnode[0]);
            ntree.push(tree[0]);
            // ntree.push(nnode[2]);

            if tree[0].unwrap() == nnode[1].unwrap() {
                ntree.push(nnode[2]);
            } else {
                ntree.push(nnode[1]);
            }

            // left side of tree is copied, right side is none
            let mut twidth = 2;
            let mut idx = 1;
            while let Some(row) = tree.get(idx..idx + twidth) {
                ntree.extend(row);
                ntree.extend(std::iter::repeat(None).take(twidth));

                idx += twidth;
                twidth *= 2;
            }

            return ntree;
        }
    }

    let mut v = vec![Some(tree[0].unwrap() + nnode[0].unwrap())];

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
    // let mut freqs = [10, 6, 5, 1, 3];
    // let mut freqs = [10, 6, 3, 3, 3];
    let mut freqs = [1, 1, 1, 1];

    // let mut freq_ptr = &mut freqs[..];

    build_min_heap(&mut freqs);

    // TODO use simple stack
    let mut two_nodes = vec![];

    let mut huffman_tree = vec![];

    let mut idx = freqs.len();

    while let Some(node) = pop_front_min_heap(&mut freqs[..idx]) {
        dbg!(node);

        idx -= 1;

        // println!("{:?}", &freqs[..idx]);

        two_nodes.push(node);

        if two_nodes.len() == 2 {
            let root = two_nodes[0] + two_nodes[1];
            let new_node = vec![Some(root), Some(two_nodes[0]), Some(two_nodes[1])];
            // when we merge the trees, we have to make sure that
            // we're merging with the minimum node
            // which might be at the root node in the tree

            // insert root into freqs min heap
            freqs[idx] = root;
            move_node_min_heap(idx, &mut freqs);
            idx += 1;

            dbg!(&freqs[..idx]);

            // freqs[idx - 1] = root;
            // move_node_min_heap(idx, &mut freqs);

            dbg!(&two_nodes);
            huffman_tree = merge_trees(huffman_tree, new_node);

            print_bt(&huffman_tree);

            two_nodes.clear();
        }

        // check if two_nodes length is 1 or 2 at the end
    }

    // merge last root node
    // if let Some(&last_node) = two_nodes.get(0) {
    //     huffman_tree = merge_trees(huffman_tree, vec![Some(last_node)]);
    // }

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
