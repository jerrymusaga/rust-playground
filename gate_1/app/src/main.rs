
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

fn fibonacci(){
    let mut nth = String::new();

    println!("Input length of your sequence");

    std::io::stdin().read_line(&mut nth).unwrap();

    let mut nth: u32 = nth.trim().parse().unwrap();

    let mut first_number = 0;
    let mut second_number = 1;

    if nth <= 0 {
        println!("Input a positive number");
    }
    else if nth == 1 {
        println!("{first_number}")
    }
    else {
        println!("Fibonacci series");
        for _i in 0..nth{
            println!("{first_number}");
            nth = first_number + second_number;
            first_number = second_number;
            second_number = nth;
        }
    }



}

fn main() {
    temp_converter();
    fibonacci();
}
