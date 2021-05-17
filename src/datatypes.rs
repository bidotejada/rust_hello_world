pub fn run() {
    let x = 1;
    let y = 2.5;

    let z: i64 = 4545454545;

    println!("max i32: {}", i32::MAX);
    println!("min i32: {}", i32::MIN);
    println!("max i64: {}", i64::MAX);
    println!("min i64: {}", i64::MIN);

    let is_active: bool = true;

    let is_greater = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
