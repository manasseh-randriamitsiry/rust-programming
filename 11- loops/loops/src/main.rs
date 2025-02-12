fn make_loop(start: u32, end: u32) {
    let mut counter = start;
    loop {
        println!("The counter is at {}", counter);
        counter += 1;
        if counter == end {
            println!("The counter reached the limit");
            break;
        }
    }
}

fn make_loop_with_remaining(start: u32, end: u32) {
    let mut counter = end;
    let mut remaining = 0;
    while counter >= start {
        println!("Counter: {}, Remaining: {}", counter, remaining);
        counter -= 1;
        remaining += 1;
    }
}

fn make_loop_with_for(start: u32, end: u32) {
    for i in start..end {
        println!("With for loop: The counter is at {}", i);
    }
}

fn main() {
    make_loop(1, 3);
    make_loop_with_remaining(4, 7);
    make_loop_with_for(8, 11);
}
