fn main() {
    let message = String::from("Hello, world!");
    let message_length = calculate_string_length(&message);
    println!("The length of '{}' is {}.", message, message_length);

    // there can be only 1 owner at time
    let message2 = message;
    println!("The meessage 2 is : {}.", message2);

    // so this code will fail because the message is empty or not defined
    // println!("The meessage is : {}.", message);

    // to solve this, we use clone here so the message2 will still be the owner
    // but the best way is using references and ownership
    let message3 = message2.clone();
    println!("The meessage 3 is : {}.", message3);
    println!("The meessage 2 after clonning is : {}.", message2);
}

fn calculate_string_length(s: &String) -> usize {
    s.len()
}