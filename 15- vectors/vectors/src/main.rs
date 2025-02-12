fn main() {
    // let vector: Vec<i32> = Vec::new();
    // let mut vector: Vec<i32> = Vec::new();
    let mut vector = vec![1, 2, 3];
    vector.push(3);
    vector.push(4 );
    println!("{:?}", vector);

    let vector2 = vector.clone();
    let vector3 = &vector2[4]; // assign the nmumber 4 in vector element to vector2
    println!("{:?}", vector3); // result is 4
    let fifth = vector.get(4); // count from 0
    match fifth{
        Some(fifth) => println!("There is a fifth element: {}", fifth),
        None => println!("There is no fifth element")
    }
}
