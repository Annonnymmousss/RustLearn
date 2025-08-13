struct User {
    active: bool,
    username: String,  
    email: String,      // inside curly brackets, we define the names and types of the pieces of data, which we call fields
    sign_in_count: u64,
}

fn main() {
    let user1 = User {          // Rust doesn’t allow us to mark only certain fields as mutable.
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,   ///when want to pass the value through the function 
        email: email,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,           //the parameter names and the struct field names are exactly the same so we can just mention those
        email,              //else we must have mentioned the parameters again like before
        sign_in_count: 1,
    }
}


//Creating Instances from Other Instances with Struct Update Syntax

fn main() {
    // --snip--
    //the struct and the instance should be here 


    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}


//The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1     //The ..user1 must come last to specify that any remaining fields should get their values from the 
    };              //corresponding fields in user1.
}//here as we have used = the ownership of user1 is transfered to user2 so user1 doesnt exist as the drop fn is called 
//but if we manually pass username and email rather than using that of user1 then user1 will still be alive
//the other parameters can be copied because they are following the copy triat so doesnt matter



//Using Tuple Structs Without Named Fields to Create Different Types

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}//its not like redular tuple that u will be access it just by destructuring it through .. and access the value like .
//u should destructure it properly using its name
//let Point(x, y, z) = origin; to destructure the values in the origin point into variables named x, y, and z.


//Unit-Like Structs Without Any Fields
//Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store
//in the type itself.
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}


//if u want to store references in a struct rather than values u need to use lifelines else it will not run.
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}//will through error of missing lifeline parameter 