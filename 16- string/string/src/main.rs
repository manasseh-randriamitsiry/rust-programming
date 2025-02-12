fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");
    let string3 = string1 + &string2; // s1 is moved here
    // let string3 = &string1 + &string2; This have an error, cannot use 2 references
    // let string3 = string1 + string2; This have an error, cannot move more than 1
    println!("{}", string3);
}
