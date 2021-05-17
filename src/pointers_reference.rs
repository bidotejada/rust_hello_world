pub fn run() {
    //Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    // cannot directly point to it if its not a primitive value
    // crete a reference
    println!("{:?}", (&vec1, vec2));
}
