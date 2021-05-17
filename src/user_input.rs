use std::io::stdin;
pub fn run() {
    let mut input = String::new();
    println!("enter a positive number:");

    // let mut test = "";
    // stdin().read_line(&mut input).unwrap();
    // test = input.trim();
    // println!("you typed: {}", test);
    // println!("length: {}", test.len());

    // let mut num1 = 0;
    // num1 = test.parse::<i32>().unwrap();
    // println!("{} + 5 = {}", num1, num1 + 5);

    /////////////////////////////Part Two/////////////////////////////////

    let mut trimmed_input: &str = " ";
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            trimmed_input = input.trim();
            println!("{} bytes read", _n);
            println!("{} bytes read", trimmed_input.len());
            println!("{:?}", trimmed_input.as_bytes());
            println!("you typed: {}", trimmed_input);
        }
        Err(error) => println!("error: {}", error),
    }
    let mut num1: u8 = 0;
    match trimmed_input.parse::<u8>() {
        // Ok(n) => println!("number: {}", n),
        Ok(n) => num1 = n,
        Err(e) => {
            println!("error: {:?}", e);
            println!("error: {}", e);
        }
    }

    println!("{} + 3 = {}", num1, num1 + 3);
    println!("max u8 {}", u8::MAX);
}
