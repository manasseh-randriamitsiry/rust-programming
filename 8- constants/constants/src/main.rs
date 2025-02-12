// cannot assign mutable with constants
// variable must be in capital letter
// must have a declared type

const YEAR: i32 = 2025;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    println!("Year: {}", YEAR);
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
