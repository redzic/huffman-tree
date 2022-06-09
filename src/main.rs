use std::{
    cmp::{Ord, PartialOrd},
    fmt::{Debug, Display},
    ops::Add,
};

pub fn pop_front_min_heap<T: PartialOrd + Clone + Ord>(x: &mut [T]) -> Option<T> {
    if x.is_empty() {
        return None;
    }

    if x.len() == 1 {
        return Some(x[0].clone());
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

    Some(ret.clone())
}

// build min heap in-place
pub fn build_min_heap<T: PartialOrd + Clone>(x: &mut [T]) {
    for idx in 0..x.len() {
        move_node_min_heap(idx, x);
    }
}

pub fn move_node_min_heap<T: Clone + PartialOrd>(mut node_idx: usize, tree: &mut [T]) {
    while node_idx != 0 {
        // TODO see if we can cache this value.
        // We move this node up the tree, but we don't need to load from memory
        // each time we move it up.
        // Easiest way to do this would probably to be add some function to
        // BinaryHeap that gets the root node.
        let node = &tree[node_idx];
        // get value of parent node
        let p_node = &tree[(node_idx - 1) / 2];
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
pub fn merge_trees<T: Copy + PartialEq + Add<Output = T>>(
    // this vec can be empty
    tree: Vec<Option<T>>,
    // this can cannot be empty
    nnode: Vec<Option<T>>,
) -> Vec<Option<T>> {
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
    // TODO use NonzeroUsize instead
    tree: Vec<Option<usize>>,
}

impl BinaryHeap {
    fn root(x: usize) -> Self {
        Self {
            tree: vec![Some(x)],
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

fn main() {
    // frequency map
    // each element is a binary tree
    let mut freqs = create_freqs![1, 2, 3, 4];

    build_min_heap(&mut freqs);

    // TODO use simple stack
    let mut two_nodes = vec![];

    let mut idx = freqs.len();

    while let Some(node) = pop_front_min_heap(&mut freqs[..idx]) {
        idx -= 1;

        two_nodes.push(node);

        if two_nodes.len() == 2 {
            // insert root into freqs min heap
            let node1 = two_nodes.pop().unwrap();
            let node2 = two_nodes.pop().unwrap();
            freqs[idx] = BinaryHeap {
                tree: merge_trees(node1.tree, node2.tree),
            };
            move_node_min_heap(idx, &mut freqs);
            idx += 1;
        }
    }

    print_bt(&freqs[0].tree);

    // TODO check if this is necessary or not?
    // merge last root node
    // if let Some(&last_node) = two_nodes.get(0) {
    //     huffman_tree = merge_trees(huffman_tree, vec![Some(last_node)]);
    // }
}
