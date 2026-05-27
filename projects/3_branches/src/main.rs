/*
fn main() {
    let number = 3; // immutable by default

    if number < 5 {
        println!("condition was true");
    } else { 
        println!("condition was false");
    }
}
 */
/* 
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }

    println!("The result is {result}");
}
*/

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}