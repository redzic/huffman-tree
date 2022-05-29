use std::ptr::swap;

pub fn remove_max_heap(x: &mut Vec<u8>) -> Option<u8> {
    // TODO handle edge cases where it's not perfectly aligned
    // also this could be empty...

    let mut idx = 0;

    // TODO fix up...
    let ret = if x.is_empty() { return None } else { x[0] };

    // swap last node and root node
    let last = x.remove(x.len() - 1);
    x[0] = last;

    loop {
        let root_node = x[idx];

        // idx points to root node

        // get child nodes
        // let c1 = x[2 * idx + 1];
        // let c2 = x[2 * idx + 2];
        let c1 = x.get(2 * idx + 1);
        let c2 = x.get(2 * idx + 2);

        match (c1, c2) {
            (Some(&left), Some(&right)) => {
                let largest_child_idx = if left >= right { 1 } else { 2 };

                let max = root_node.max(left).max(right);
                if root_node != max {
                    // need to swap with largest in that case
                    // and traverse down the tree
                    unsafe { swap(&mut x[idx], &mut x[2 * idx + largest_child_idx]) }

                    // adjust index, go to child node that was larger
                    idx = 2 * idx + largest_child_idx;
                } else {
                    break;
                }
            }
            (Some(&left), None) => {
                // compare last child node with parent node
                // since this is a max heap, the child node should not be greater than
                // the parent
                if left > root_node {
                    unsafe {
                        swap(&mut x[idx], &mut x[2 * idx + 1]);
                    }
                }
                break;
            }
            (None, Some(_)) => unsafe { std::hint::unreachable_unchecked() },
            (None, None) => break,
        }
    }

    Some(ret)
}

pub fn build_max_heap(x: &[u8]) -> Vec<u8> {
    let mut output = vec![0u8; x.len()];

    for (mut idx, &c_node) in x.iter().enumerate() {
        output[idx] = c_node;

        loop {
            if idx == 0 {
                break;
            }
            let p_node = output[(idx - 1) / 2];
            // check if parent node is greater than current node
            if p_node < c_node {
                unsafe {
                    // swap current node with parent node
                    swap(&mut output[idx], &mut output[(idx - 1) / 2]);
                }
                // set index to parent node
                idx = (idx - 1) / 2;
            } else {
                // else no swapping required
                break;
            }
        }
    }

    output
}

fn main() {
    // let chars: Vec<u8> = (0..32).collect();
    // let chars = [1u8, 5, 6, 7, 8, 2, 1, 3, 9, 1];
    let chars = [1u8, 5, 6, 7, 8, 2, 1];

    let mut bt = build_max_heap(&chars);

    println!("bt before: {:?}", bt);

    // correct: 1 5 1 7 8 6 2 3

    for _ in 0..3 {
        println!("{:?}", remove_max_heap(&mut bt));
        println!("bt: {:?}", bt);

        println!();
    }
}
