
fn main() {
    value_scope();

    string_example();

    move_i32_example();

    move_string_example();

    scope_and_assignment();

}

/*

ownship rules:
1、every value has an owner.
2、There is only one owner at a time.
3、When the owner goes out of scope, the value will be dropped.
 */ 

fn value_scope() {  // snake_case
    // s is not valid, because it is not declared.
    let s = "hello";  // s is valid.
    println!("s is {}", s);
} // s goes out of scope, hello will be dropped.

// String type
fn string_example() {
    let mut s = String::from("hello"); // create a String variable from a string literal. It is mutated.
    s.push_str(" world");
    println!("String type s has a value of {}", s);
} // Rust call a special function to free s, it is drop.

// memory and allocation
// Two element: request memory at runtime;
fn move_i32_example() {
    let x = 5;
    let y = x; // copy trait
    println!("x is {} and y is {}", x, y);
}

/*

41 |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
42 |     let s2 = s1;
   |              -- value moved here
43 |     println!("s1 is {} and s2 is {}", s1, s2);
   |                                       ^^ value borrowed here after move
*/
fn move_string_example() {
    let s1 = String::from("hello");

    let s2 = s1; // ownership moves from s1 to s2,  shallow copy, use clone() to deep copy

     //  value borrowed here after move
    // let s3 = s1.clone(); 

    println!("s2 is {}", s2);

    //  println!("s3 is {}", s3);

    // After ownership moves, it won`t work when you use s1.
    // println!("s1 is {} and s2 is {}", s1, s2); 
}

fn scope_and_assignment() {
    let mut s1 = String::from("hello");

    println!("s1 is {}", s1);

    s1 = String::from("new value"); // hello goes out of scope, Rust will drop it

    println!("{} world", s1);
}

// copy trait: integer、boolean、float、char、 tuple only with copy trait type.

fn takes_and_gives_back() -> string {
    let some_thing = String::from("hello");
    some_thing
}