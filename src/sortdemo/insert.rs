pub fn sort(arr: Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    let mut ret = Vec::new();
    while i < arr.len() {
        let mut j = 0;
        let vi = arr.get(i).unwrap();
        while j < ret.len() {
            if let Some(vj) = ret.get(j) {
                if vi < vj {
                    break;
                }
            }
            j = j + 1;
        }
        ret.insert(j, *vi);
        i = i + 1;
    }
    return ret;
}
