fn main() {
    // let x = 65; // Here x is immutable by default.
    let mut x = 65; // Here x is mutable as specified.
    println!("The value of x is {x}");
    x = 129;
    println!("The value of x is {x}");
}
