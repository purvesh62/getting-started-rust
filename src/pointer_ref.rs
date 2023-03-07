pub fn run() {

    // Primitive array
    let arr1 = [1,2,3,4];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));


    // Vector non-primitive array
    // If assigned to another variable, the first variable will no longer hold the value.
    // You'll need to use a reference (&) to point to the resource

    let vec1 = vec![1,2,3,4];
    let vec2 = &vec1;

    // To use vec1 you'll have to use "&" before vec1. 
    println!("{:?}", (&vec1, vec2));
}