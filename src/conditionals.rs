pub fn run() {
    let age: u8 = 21;
    let check_id: bool = true;
    let knows_person_age = false;

    if age >= 21 && check_id || knows_person_age {
        println!("you can drink");
    } else if age < 21 && check_id {
        println!("you can't drink");
    } else {
        println!("can i see ID");
    }

    //shorthand if
    let is_of_age: bool = if age >= 21 { true } else { false };
    println!("of age:{}",is_of_age);
}
