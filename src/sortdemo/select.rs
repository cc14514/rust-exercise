pub fn sort(mut list: Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    let mut ret: Vec<u32> = Vec::new();
    while i < list.len() {
        let mut vali: &u32 = &0;
        if let Some(v) = list.get(i) { vali = v }
        let list2 = Vec::from(&list[i + 1..]);
        let mut list3 = Vec::new();
        let mut j = 0;
        while j < list2.len() {
            let mut valj: &u32 = &0;
            if let Some(v) = list2.get(j) { valj = v }
            j = j + 1;
            if valj < vali {
                list3.push(*vali);
                vali = valj;
            }else{
                list3.push(*valj);
            }
        }
        ret.push(*vali);
        list = Vec::from(ret.as_slice());
        list.append(&mut list3);
        i += 1;
    }
    return ret;
}