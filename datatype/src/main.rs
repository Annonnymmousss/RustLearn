fn main() {
let x = 2.0; // f64
let y: f32 = 3.0; // f32

//numeric operation 
// addition
let sum = 5 + 10;
// subtraction
let difference = 95.5 - 4.3;
// multiplication
let product = 4 * 30;
// division
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // Results in -1
// remainder
let remainder = 43 % 5;


//boolean type 
let t = true;
let f: bool = false; // with explicit type annotation

//tuple type 
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {y}");
//we can access any value as 
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;


//array type 
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];   //[3,3,3,3,3]
//Accessing Array Elements
let first = a[0];
let second = a[1];
// if u try to access elements more than the length of array it will show runtime error

}
