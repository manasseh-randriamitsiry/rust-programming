fn main(){
    let _x = 5;
    // we cannot assign another value to x anymore because by default a variable is immutable in rust
    // so this will fail : let x = 6;
    // instead use
    let mut _y = 5;
    println!("x before = {}", _y);
    _y = 6;
    println!("x now = {}", _y);
}