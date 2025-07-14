fn main() {
println!("Hello, world!");
another_function();
another_function1(10);
print_labeled_measurement(5 , 'H');
let x = five();
println!("The value of x is: {x}");

//statements and expressions 
let y = {
let x = 3;
x + 1  //their is no semicolon because if we put it , it will be a statemenet and will not return anything
};
println!("The value of y is: {y}");
}

fn another_function() {
println!("Another function.")
}

fn another_function1(x: i32) {
println!("The value of x is: {x}");  //passign parameters
}

fn print_labeled_measurement(value: i32, unit_label: char) {
println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
5
} // it will return a integer of 32 bit which is 5 
