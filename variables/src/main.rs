mod functions;
mod ownership;

fn main() {
    
    variables();
    functions::main();
    ownership::main();
}

fn variables() {
    mut_const_example();
    shadow_example();
    integer_types();
    char_type();
    bool_type();
    float_types();
    numeric();
    compound_types();
    expressions_statements();
}

fn expressions_statements() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn mut_const_example() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

fn shadow_example() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("Spaces Value: '{spaces}'");
    let spaces = spaces.len();
    println!("Spaces Length: {spaces}");
}

/**
 * 
 * Length	Signed	Unsigned
 * 8-bit	i8	u8
 * 16-bit	i16	u16
 * 32-bit	i32	u32
 * 64-bit	i64	u64
 * 128-bit	i128	u128
 * arch	isize	usize
 * 
 */
fn integer_types() {
    let x : i8 = -12;
    let y : u16 = 240;
    println!("The value of x is: {x}");
    println!("The value of x is: {y}");

    let x : u8 = 128;
    println!("The value of x is: {x}");
}

fn char_type() {
    let c = 'z'; // four bytes in size
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Char types: {},{},{}", c, z, heart_eyed_cat);
}

fn bool_type() {
    let t = true;
    let f : bool = false;
    println!("Boolean Values: {} and {}", t, f);
}

fn float_types() {
    let x= 2.01; //f64
    let y : f32 = 3.02;
    println!("The value of x is: {x}");
    println!("The value of x is: {y}");
}

fn numeric() {
    // addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Product: {product}");
    
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");
    
    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");
}

fn compound_types() {
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, z) = tup; // underscore variables if not used
    println!("Value of middle variable: {y}");
    println!("Value of first variable: {}", tup.0);
    println!("Value of last variable: {} {}", tup.2, z);

    let a: [i32; 5] = [1,2,3,4,5];
    println!("Value of array: {:?}", a);
    let a = [3; 5];
    println!("Value of array: {:?}", a);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("Months: {:?}", months);
    println!("First Month: {}", months[0]);
    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()


}