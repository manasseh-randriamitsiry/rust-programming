// enum Option<T>{
//     None,
//     Some(T)
// }

// enum Result<T>{
//     Ok(T),
//     Err(String)
// }

// using option
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    }else {
        Some(numerator / denominator)
    }
}
// using result
fn divide_result(numerator: f64, denominator: f64) -> Result<f64,String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else { Ok(numerator / denominator) }
}

fn main() {
    let result = divide_option(10.0, 2.0);
    match result{
        Some(value) => println!("The result is {}", value),
        None => println!("Can't divide by 0")
    }

    let result2 = divide_result(10.0, 0.0);
    match result2{
        Ok(value) => println!("The result is {}", value),
        Err(e) => println!("Error: {}", e)
    }
}
