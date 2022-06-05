pub fn pop_front_min_heap(x: &mut [u8]) -> Option<u8> {
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
pub fn build_min_heap(x: &mut [u8]) {
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

fn main() {
    // sorted frequency map
    // a, b, c, d, e, ... etc
    let mut freqs = [2, 4, 4, 5, 5];

    build_min_heap(&mut freqs);

    let mut y = &mut freqs[..];
    while let Some(x) = pop_front_min_heap(y) {
        dbg!(x);

        let len = y.len();
        y = &mut y[..len - 1];
    }

    println!("{freqs:?}");
}
