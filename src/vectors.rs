use std::mem;

pub fn run() {
    let mut numbers: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut list: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", numbers);
    println!("{:?}", list);
    list[1] = -25;
    println!("{:?}", list);

    //get single value
    println!("{}", numbers[0]);

    //get vector length
    println!("numbers len:{}", numbers.len());

    // println!("memory of numbers:{} bytes",std::mem::size_of_val(&numbers));
    println!("memory of numbers:{} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[u32] = &numbers;
    let slice: &[u32] = &numbers[0..2];

    //add to vector
    list.push(4);
    println!("{:?}", slice);
    list[1]=2;
    println!("{:?}", list);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);

    //loop through vector
    for x in numbers.iter(){
        println!("number:{}",x);
    }

    for x in list.iter_mut(){
        *x *= 2;
    }
    println!("list Vec:{:?}",list);
}
