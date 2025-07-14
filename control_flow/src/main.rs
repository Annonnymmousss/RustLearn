fn main() {
let number = 6;
if number % 4 == 0 {
println!("number is divisible by 4");
} else if number % 3 == 0 {
println!("number is divisible by 3");
} else if number % 2 == 0 {
println!("number is divisible by 2");
} else {
println!("number is not divisible by 4, 3, or 2");
}


let condition = true;
let number = if condition { 5 } else { "six" };// it will show compile time error because in if statement both in if and else the datatype must be same 
} // if is a expression so when use it with a let u have to make it like a return type
