fn main() {
    let array: [u32; 5] = [1,2,3,4,5];

    let mut index = String::new();

    println!("Input an index");

    std::io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Not a number");

    let element = array[index];

    println!("element {element} has index {index}");
    
}
