// arrays. tuples, slices, strings (slices string)
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array {:?}", numbers);

    // for string arrays
    // the types is defined automatic by rust so from here
    // not need something like  [i32; 5]
    let weekday = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("Weekday Array {:?}", weekday);
    println!("Weekday Array 2nd element: {:?}", weekday[1]);
    // using println!("Weekday Array 2nd element: {}", weekday[1]); works too

    // for Tuples
    let random_tuple = (1, "Hello", 3.14);
    println!("Random Tuple {:?}", random_tuple);
    //Slices
    let number_slice = &numbers[0..5]; // work also &[0,1,2,3,4,5]
    println!("Number Slice {:?}", number_slice);

    // Strings [ growable, mutable, owned]
    let mut hello = String::from("Hello");
    hello.push_str(", World!");
    hello.push_str(",another push");

    // String slice
    let str_slice = &hello[0..5];
    println!("String Slice {:?}", str_slice);
}