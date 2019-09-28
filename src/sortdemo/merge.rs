pub fn sort(list: Vec<u32>) -> Vec<u32> {
    let mut src = list.clone();
    store_fn(&mut src, 0, list.len() - 1);
    return src;
}

fn merge_fn(arr: &mut Vec<u32>, l: usize, mid: usize, r: usize) {
    let mut idx: Vec<u32> = Vec::new();
    let mut i = l;
    let mut j = mid + 1;
    while i <= mid && j <= r {
        if arr[i] < arr[j] {
            idx.push(arr[i]);
            i = i + 1;
        } else {
            idx.push(arr[j]);
            j = j + 1;
        }
    }
    while i <= mid {
        idx.push(arr[i]);
        i = i + 1;
    }
    while j <= r {
        idx.push(arr[j]);
        j = j + 1;
    }
    //println!("{:?}", idx);

    let mut i = 0;
    while i < idx.len() {
        arr[l + i] = *idx.get(i).unwrap();
        i = i + 1;
    }
}

fn store_fn(arr: &mut Vec<u32>, l: usize, r: usize) {
    //println!("store --> {},{},{:?}", l, r, arr);
    if l == r {
        return;
    }
    let mid = l + ((r - l) >> 1);
    store_fn(arr, l, mid);
    store_fn(arr, mid + 1, r);
    merge_fn(arr, l, mid, r);
}

