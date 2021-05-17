use std::mem;

pub fn run(){

    let numbers:[u32;5]=[1,2,3,4,5];
    let mut list=[1,2,3];

    println!("{:?}",numbers);
    println!("{:?}",list);
    list[1]=-25;
    println!("{:?}",list);

    //get single value
    println!("{}",numbers[0]);

    println!("numbers len:{}",numbers.len());

    // println!("memory of numbers:{} bytes",std::mem::size_of_val(&numbers));
    println!("memory of numbers:{} bytes",mem::size_of_val(&numbers));

    //get slice
    let slice:&[u32]=&numbers;
    let slice:&[u32]=&numbers[0..2];
    println!("{:?}",slice);
}