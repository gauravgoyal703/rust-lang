
pub fn main() {

    println!("Inside Module Functions");

    param(five());
    print_labeled_measurement(16, 'h');
    if_intro();
    loop_return();
    multiple_loops();
    while_for_test();
}

fn five() -> i32 {
    5
}

fn param(x: i32) {
    println!("The value of x: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn if_intro() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

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
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            // add the return expression after the break statement
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining  = 10;

        loop {
            println!("Remaining: {remaining}");

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
    println!("Final Count: {count}");
}

fn while_for_test() {
    let mut count = 0;

    while count < 5 {
        count += 1;
    }

    println!("While Count: {count}");

    let arr = [10, 20, 30, 40];

    for el in arr {
        print!("Element: {el}, ");
    }
    println!("");

    for el in 0..5 {
        print!("Element: {el}, ");
    }
    println!("");
}