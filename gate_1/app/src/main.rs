
fn temp_converter(){
    println!("Which Operation to perform\n 1. Convert to Centigrade\n 2. Convert to Fahrenheit");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u32 = input.trim().parse().unwrap();
       
    let mut number_to_convert = String::new();

    println!("Input a value to convert");

    std::io::stdin().read_line(&mut number_to_convert).expect("Failed to read line");

    let num: f64 = match number_to_convert.trim().parse() {
        Ok(n) => n,
        Err(_) => return
    };

    if input == 1 {
        to_centigrade(num);
    }else if input == 2 {
        to_fahrenheit(num);
    }else {
        println!("Input should either be 1 or 2");
    }

}

fn to_fahrenheit(number: f64) -> f64 {
    let output = (number * 9.0/5.0) + 32.0;
    println!("{}", output);
    output
}

fn to_centigrade(number: f64) -> f64 {
    let output = (number - 32.0) * 5.0/9.0;
    println!("{}", output);
    output
}

fn main() {
    temp_converter();
}
