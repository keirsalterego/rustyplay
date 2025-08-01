fn main() {
    single_parameter(5);
    multi_parameter(3, 2);
}

// Single parameter function

fn single_parameter(x: i32) {
    println!("The value of x is: {x}");
}

// Multi Parameter function

fn multi_parameter(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

