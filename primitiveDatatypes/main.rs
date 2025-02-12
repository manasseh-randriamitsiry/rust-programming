fn main(){
    // i8, i16, i32, i64, i128: signed integers
    // u8. u16, u32, u64, u128: unsigned, for just a number without sign - or +
    let x: i32 = -10;
    let y: u64 = 100;
    println!("Signed integer: {} ",x);
    println!("unsigned integer: {} ",y);

    // Floats : f32, f64
    let pi: f32 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean values same as C
    let is_rust_ok: bool = true;
    println!("Is rust ok ? {}", is_rust_ok);

    // char
    let letter: char = 'a';
    println!("The caracter is: {}", letter);

}