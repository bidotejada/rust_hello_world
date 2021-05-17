pub fn run() {
    let name = "willy";
    let mut age = 37;
    age = 26;
    println!("my name is {} and i am {} years old", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("willy", 26);
    println!("{} is {}",my_name,my_age);
}
