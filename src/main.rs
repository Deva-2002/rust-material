// fn main() {
//     println!("Hello, world!");
// }

//create a guessing game, take user input and compare it to the selected value
// use rand::Rng;
// use std::{cmp::Ordering, io};
// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("your secret number is {secret_number}");
//     loop {
//         let mut guess = String::new();
//         println!("please input your guess");
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("failed in taking input"); -- to catck the error
//         println!("your guess is: {guess}");
//         let guess: u32 = guess.trim().parse().expect("please enter a number"); -- convert to number
//         match guess.cmp(&secret_number) {
//             Ordering::Less => {
//                 println!("it is less")
//             }
//             Ordering::Equal => {
//                 println!("you win");
//                 break;
//             }
//             Ordering::Greater => {
//                 println!("it is greater");
//             }
//         }
//     }
// }

//cargo.lock - store exact verion of dependencies your project is build in 
// (so it remains constant across development untill you explicitly change it)

//cargo update - to update the dependencies


//cannot do this since by default every variable in rust is immutable
//variables are immutable by default, you can make them mutable by adding mut in front of the variable name
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

//this will work
// fn main(){
//     let mut x=6;
//     println!("x is: {x}");
//     x=10;
//     println!("x is: {x}");
// }


//constants-- you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable
//ex: const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

//shadowing --you can declare a new variable with the same name as a previous variable. 
// Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

// fn main(){
//     let x=5;
//     let x=String::from("hello"); //type is not a problem when shadowing
//     let x: i32=6;
//     {
//         let x=10;
//         println!("x is: {x}");
//     }
//     //x inside the {} is alos shadowing hte original x, its called inner shadowing and it end inside the inner lopp
//     println!("x is: {x}"); // this will be 6 (because scope);
// }

//Data Types
// Scalar Types
// A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

// fn main(){
//     let guess:u32="42".trim().parse().expect("cant be a number");
//     println!("guess is: {guess}");
// }

// integer Types
// Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16    	u16
// 32-bit	i32	    u32
// 64-bit	i64	    u64
// 128-bit	i128	u128
// architecture dependent	isize	usize

// i8 - stores -2^7 to 2^7-1 (ie -127 to 128);
// u8 - stores 0 to 256 (0 to 2^8)

// Additionally, the isize and usize types depend on the architecture of the computer your program is running on:
// 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

//floating point
// fn main() {
//     let x = 2.0; // f64(by deafult)

//     let y: f32 = 3.0; // f32
// }

//boolean
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

//characters
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
//     println!("{heart_eyed_cat}");
// }


// Compound Types
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

// tuples--A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// Tuples have a fixed length: once declared, they cannot grow or shrink in size

// fn main(){
//     let tup:(u32,String,bool)=(33,String::from("deva"),true);
//     println!("{tup:?}")
// }

// fn main(){
//     let tup:(f32,bool,char)=(2.0,true,'x');
//     let(a,b,c)=tup;
//     println!("{a},{b},{c}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

//     println!("{five_hundred},{six_point_four},{one}");
// }

// The Array Type
// Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
// Unlike arrays in some other languages, arrays in Rust have a fixed length.

// fn main(){
//     let arr=[1,2,3,4,5];
//     let arr2:[i32;4];
//     arr2=[1,2,3,4];
//     println!("{arr:?},{arr2:?}")
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];

//     println!("{first},{second}");
// }

// use std::io;
// fn main(){
//     let arr=[1,2,3,4,5];
//     println!("enter an index value");
//     let mut index=String::new();
//     io::stdin().read_line(&mut index).expect("invalid input");
//     let index:usize=index.trim().parse().expect("error while parsing");
//     let element=arr[index];
//     println!("element at index {index} is : {element}");
// }