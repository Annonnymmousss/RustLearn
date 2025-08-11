//Each value in Rust has an owner.
//There can only be one owner at a time.
//When the owner goes out of scope, the value will be dropped.

fn main() {
    let s1 = "hello";//global scope


 
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    
    // let s2 = s1+s or s1 + "cdsc"
    // println!(s2)     not possible as string literals are immutable
    

    let s2 = String::from("hello");  //it allocates data in heap

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

}