pub fn run() {
    println!("hello from print.rs file");
    println!("{}", 1);
    println!("{} is from {}", "will", "klk");
    println!("{0} is from {1} and {0}", "will", "klk");
    println!("{name} is from {city} and {do}",name="willy",city="bonao",do=25);
    println!("{:b} {:x} {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "hello"));
    println!("10 + 10 = {}", 10 + 10);
}
