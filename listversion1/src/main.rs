use listversion1::first::List;
fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("list = {:?}", list);
}
