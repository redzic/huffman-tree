use rand::prelude::*;

pub fn pop_front_max_heap(x: &mut [u8]) -> Option<u8> {
    let mut idx = 0;

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

    loop {
        let root_node = x[idx];

        // idx points to root node

        // get child nodes
        let c1 = x.get(2 * idx + 1);
        let c2 = x.get(2 * idx + 2);

        match (c1, c2) {
            (Some(&left), Some(&right)) => {
                let largest_child_idx = if left >= right { 1 } else { 2 };

                let max = *[left, right, root_node].iter().max().unwrap();
                if root_node != max {
                    // need to swap with largest in that case
                    // and traverse down the tree
                    x.swap(idx, 2 * idx + largest_child_idx);

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
                    x.swap(idx, 2 * idx + 1);
                }
                break;
            }
            _ => break,
        }
    }

    Some(ret)
}

// build max heap in-place
pub fn build_max_heap(x: &mut [u8]) {
    for mut idx in 0..x.len() {
        let c_node = x[idx];

        loop {
            if idx == 0 {
                break;
            }
            let p_node = x[(idx - 1) / 2];
            // check if parent node is greater than current node
            if p_node < c_node {
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

// Maybe write heap sort in assembly? for i32 or something maybe.
pub fn heap_sort(mut x: &mut [u8]) {
    // build into max heap
    build_max_heap(x);

    while let Some(v) = pop_front_max_heap(x) {
        let len = x.len();
        x[len - 1] = v;
        x = &mut x[..len - 1];
    }
}

fn main() {
    // let chars: Vec<u8> = (0..32).collect();
    // let chars = [1u8, 5, 6, 7, 8, 2, 1, 3, 9, 1];
    let cap = 4096;

    let mut buf1 = vec![0u8; cap];
    let mut buf2 = vec![0u8; cap];

    let mut rng = rand::thread_rng();
    loop {
        let len = rng.gen_range(0..cap);

        buf1[..len].fill_with(|| rng.gen());
        buf2[..len].copy_from_slice(&buf1[..len]);

        heap_sort(&mut buf1[..len]);
        buf2[..len].sort();

        assert_eq!(buf1[..len], buf2[..len]);
    }
}
