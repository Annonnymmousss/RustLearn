fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //constants
    let mut x = 5; //made mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //we can change datatype by shadowing but not by making it mutable
    let spaces = "   ";
    let spaces = spaces.len();//this is valid

    let mut spaces = "   ";
    
    spaces = spaces.len();//this isnt 
}