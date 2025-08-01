fn main() {

    [lspconfig] cmd failed with code 101: table: 0x7ff0a5fd49d0                                                                                                                                                                                
error: current package believes it's in a workspace when it's not:                                                                                                                                                                      
}

fn example1() {
    let x = 5;
    println!("x is {}", x); // we can print x 
}

fn example2() {
    let x = 2; // x owns the value 2
    let y = x; // y owns the value 2 now 
    //
    println!("x is {}", x); // we can print x
    println!("y is {}", y); // we can print y  
}
