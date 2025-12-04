mod mybox;

fn main() {
    let x = 42;
    let y = 42;

    println!("x value: {}", x);
    println!("x address: {:p}", &x); // The '&' operator asks for the memory location
    println!("y address: {:p}", &y);
}
