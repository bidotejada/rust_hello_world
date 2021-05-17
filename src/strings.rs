pub fn run() {
    let hello = "hello";
    let mut world = String::from("world");

    world.push('.');
    world.push_str(" and friends.");

    // println!("{:?}", (hello, world));
    println!(
        "'world' -> length:{}\ncapacity:{}\n is empty:{}\ncontains '. and':{}",
        world.len(),
        world.capacity(),
        world.is_empty(),
        world.contains(". and")
    );

    println!("replace: {}",world.replace("friends", "Amigos"));
    for word in world.split_whitespace(){
        println!("{}",word);
    }
    let mut s=String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}",s);

    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());

}
