use rustexercise::sortdemo;

fn main() {
    println!("hello world.");
    rustexercise::looptest();
    let list: Vec<u32> = vec![11, 4, 2, 3, 1, 5, 8, 2, 3, 5, 6, 7, 9, 10];
    println!("list = {:?}", list);
    let mut list = sortdemo::select::sort(list);
    println!("select.sort = {:?}", list);
    list.reverse();
    println!("list = {:?}", list);
    let mut list = sortdemo::insert::sort(list);
    println!("insert.sort = {:?}", list);

    list.reverse();
    println!("list = {:?}", list);

    let list = sortdemo::merge::sort(list);
    println!("merge.sort = {:?}", list);
}