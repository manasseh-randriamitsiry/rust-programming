// simple function return string
fn hello_rust() {
    println!("Hello, from rust");
}
// define function and return calculated height
// used unsigned because there is no minus height negative
fn height_calculator(height: u32) -> f32 {
    height as f32 / 100.0
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(height: u32, weight: u32) -> f32 {
    let height_in_meters = height_calculator(height);
    weight as f32 / (height_in_meters * height_in_meters)
}

// function with multiple parameters using another functions
fn human_id(name: &str, age: u8, height: u32) {
    println!("Hello, my name is {}, \
    i am {} old, \
    my height is {}", name,age,height_calculator(height));
}

// Entry point
// All the functions used must be on top
fn main() {
    // calling function
    hello_rust();
    // calling and using function
    println!("Height in meters: {}", height_calculator(180));

    // calling functions with multiple parameters
    human_id("John", 25, 180);

    // calculate
    println!("addition: {}", add_numbers(1,5));
    let bmi = calculate_bmi(180, 80);
    println!("BMI: {:.2}", bmi);
}
