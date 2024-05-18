pub fn main() {

    println!("Inside Module Ownership");
    
    value_borrowed_string_move_example();
    ownership_example();
    retain_ownership();
    retain_ownership_with_ref();
    mut_example();
}

fn value_borrowed_string_move_example() {
    let s1 = String::from("hello");
    // let s2 = s1; // The value is moved from s1 to s2 and this makes s1 not usable. 
                    //We get an error if we try to use s1 after this.

    let _s2 = s1.clone();

    println!("Value: {s1}");
}

fn ownership_example() {
    let s = String::from("Hello");
    take_ownership(s);
    //println!("I can't see s: {s}"); // This will through error

    let x = 5;
    makes_copy(x);
    println!("I confirm that x is a copy: {x}");
}

fn take_ownership(str: String) {
    println!("This string is mine now: {str}");
}

fn makes_copy(x: u32) {
    println!("This value is copied: {x}");
}

fn retain_ownership() {

    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len() // len() returns the length of a String
}

fn retain_ownership_with_ref() {

    let s1 = String::from("World");

    let len = calculate_length_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);

}

fn mut_example() {

    let mut s = String::from("Hello");

    change(&mut s);

    println!("Value: {s}");

}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

