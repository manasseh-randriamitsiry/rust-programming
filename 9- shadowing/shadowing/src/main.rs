// shadowing
fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);
    {
        let x = x*5;
        println!("The value of x inside the scope is: {}", x);
        // we got 30
    }

    println!("The value of x outside the scope is: {}", x); // still 6
}

