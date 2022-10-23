fn main() {
    let mut lists = Vec::with_capacity(17);
    lists.push(5);
    lists.push(40);
    println!("{:?}", lists);
    println!("{}", lists.len());
    println!("{}", lists.capacity());
}
