fn main() {
    //Rust variables are immutable by default but can become mutable by adding the mut keyword
    let mut x =5;
    println!("The value of x is {x}");
    x = 15;
    println!("The value of x is {x}");

    //constants are always immutable
    //Rust’s naming convention for constants is to use all uppercase with underscores between words. 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}")
}
