fn main() {

    
//loop
let mut count = 0;
'counting_up: loop {     // we can declare such labels to use break or continue as we are allowed to use it in only inner loops
    println!("count = {count}");
    let mut remaining = 10;
    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
        break;
    }
    if count == 2 {
    break 'counting_up;
    }
remaining -= 1;
}
count += 1;
}
println!("End count = {count}");


//while loops
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
println!("LIFTOFF!!!");


//for loop
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {element}");
}
}