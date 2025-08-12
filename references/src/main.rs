//The issue with the tuple code in Listing 4-5 is that we have to return the String to the calling function so we can still use 
//reference to the String value. A reference is like a pointer in that it’s an address we can follow to access the data stored at 
//the String after the call to calculate_length, because the String was moved into calculate_length. Instead, we can provide a 
//that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of 
//a particular type for the life of that reference.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.


// if we try to modify something we’re borrowing?
//it doesn’t work!
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

//now its mutable referencing 
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//we cannot make two mutable references of the same thing but infinite amount of immutable referencing
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");



//put it insdie the scope to make two mut references
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;





    //u can not make immutable referecing and mutabel referencing together
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");




//drop func is called after mut reference comes into play 
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
