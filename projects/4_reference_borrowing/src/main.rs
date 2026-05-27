// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2);
//     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
// }

// fn greet(g1: String, g2: String) {
//     println!("{} {}!", g1, g2);
// }

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }

let mut x: Box<i32> = Box::new(1);
let a: i32 = *x; // *x reads the heap value of x, meaning 1
*x += 1; // x points to value 2 now

let r1: &Box<i32> = &x; // r1 points to x on the stack
let b: i32 = **r1; // meaning 1

let r2: &i32 = &*x;
let c: i32 = *r2;