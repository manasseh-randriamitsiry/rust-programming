fn age_control_flow(age: u8) {
    if age < 18 {
        println!("You are too young to vote.");
    } else if age < 65 {
        println!("You are old enough to vote.");
    } else {
        println!("You are too old to vote.");
    }
}
/*
The important part is:
    if number % divisor == 0 {
        println!("Yes,{} is divisible by {}", number, divisor);
        ...
    }
And yes, the loop also is important
*/
fn is_divisible(number: u32) {
    let mut found = false;

    for divisor in 2..=9 {
        if number % divisor == 0 {
            println!("Yes,{} is divisible by {}", number, divisor);
            found = true;
        }
    }

    if !found {
        println!("{} is not divisible by any number from 2 to 9", number);
    }
}

/*
The important part is:
   let number_bool = if found { 1 } else { 0 };
   which showing number depending on the result
*/
fn check_divisibility(number: u32) {
    let mut found = false;
    for divisor in 2..=9 {
        if number % divisor == 0 {
            found = true;
            let number_bool = if found { 1 } else { 0 };
            println!("result: {}, {} is divisible by {}", number_bool, number, divisor);
        }
    }

    if !found {
        println!("{} is not divisible by any number from 2 to 9", number);
    }
}
fn main() {
    age_control_flow(12);
    is_divisible(12);
    check_divisibility(1);
}
