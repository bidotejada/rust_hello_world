pub fn run() {
    let mut count = 0;

    //infinite loop
    loop {
        count += 1;
        println!("number:{}", count);

        if count == 20 {
            break;
        };
    }
    
    //while loop
    let mut decision = true;
    while decision {
        count -= 1;
        println!("count:{}", count);
        if count == 0 {
            decision = false;
        }
    }

    for x in 0..101 {
        if (x % 10 == 0) && (x / 10 <= 0) {
            continue;
        } else if x % 10 == 0{
            print!("number:{}, ", x);
        }
    }
}
