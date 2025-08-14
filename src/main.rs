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

// functions
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn five() -> i32 {
//     5 //this means return 5; -- (you can just put 5 or return 5;)
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// if - control statements
// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// if-else
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Using if in a let Statement
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

//while loop
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

//looping through array;
// fn main(){
//     let arr=[1,2,3,4,5];
//     let mut index=0;
//     while index!=arr.len(){
//         let val=arr[index];
//         println!("{val}");
//         index=index+1;
//     }
// }

//2nd way
// fn main(){
//     let arr=[1,2,3,4,5];
//     for elements in arr {
//         println!("{elements}");
//     }
// }

//printing numbers
// fn main(){
//     for number in 1..4{
//         println!("{number}");
//     }

//     for number in (1..4).rev(){
//         println!("{number}");
//     }
// }

//Ownership is Rust’s feature that enables Rust to make memory safety guarantees without needing a garbage collector.
//Ownership is a set of rules that govern how a Rust program manages memory
//Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.
//Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks
//The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out
//Adding data is called pushing onto the stack, and removing data is called popping off the stack.
//All data stored on the stack must have a known, fixed size.
//Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
//The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap
//Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
//Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
//Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.
//When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack
//Ownership Rules
// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// {                      // s is not valid here, since it's not yet declared
//     let s = "hello";   // s is valid from this point forward

//     // do stuff with s
// }                      // this scope is now over, and s is no longer valid

// In other words, there are two important points in time here:

// When s comes into scope, it is valid.
// It remains valid until it goes out of scope.

//string literal -- it is static (used if we know that size of string is fixed)
//it is faster and efficent
// fn main(){
//     let  s="hello";
//     // s.push_str("world"); -- you cant do this even if you make it mutable
//     println!("{s}");
// }

//this a different type-String(this is stored in heap, since its value can change during compilation);
//String::from -- this is requesting memory from heap
// fn main(){
//     let mut s=String::from("hello");
//     s.push_str(" World");
//     println!("{s}")
// }

// {
//     let s = String::from("hello"); // s is valid from this point forward

//     // do stuff with s
// }                                  // this scope is now over, and s is no
// longer valid

//We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
// We now have two variables, x and y, and both equal 5

//    let x = 5;
//   let y = x;

//its different in this case
// let s1 = String::from("hello");
// let s2 = s1;

//A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack.
// On the right is the memory on the heap that holds the content
//The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator
//https://doc.rust-lang.org/book/img/trpl04-01.svg

//When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
// We do not copy the data on the heap that the pointer refers to
// https://doc.rust-lang.org/book/img/trpl04-02.svg

// if they copied whats in heap, it will become a very expensive operation
// https://doc.rust-lang.org/book/img/trpl04-03.svg

// To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
// that means you can no longer do this

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!");
//you can print s2
// }

//With only s2 valid, when it goes out of scope it alone will free the memory, and we’re done.

// When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately
// let mut s = String::from("hello");
// s = String::from("ahoy");

// println!("{s}, world!"); -- wil print ahoy world
// https://doc.rust-lang.org/book/img/trpl04-05.svg

//If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
//now s1 and s2 can be used
//    let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");

// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");
// }

// The reason is that types such as integers that have a known size at compile time are stored entirely on the stack,
//  so copies of the actual values are quick to make

// Ownership and Functions
// The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
// fn main(){
//     let s=String::from("deva");
//     print_string(s);
//     // println!("{}",s); // will not work since transfer of ownership
//     let y=7;
//     print_int(y);
//     println!("{}",y); // this works since stored in stack
// }

// fn print_string(s:String){
//     println!("{}",s)
// }

// fn print_int(y:i32){
//     println!("{}",y);
// }

//Transferring ownership of return values

// fn main(){
//     let s=String::from("deva");
//     let (s,len)=get_len(s);
//     println!("size of {s} is: {len}");
// }

// fn get_len(s:String)->(String,usize){
//     let length=s.len();
//     return (s,length);
// }
//this is called tranferring of ownership -- to much work , to overcome it rust has a concept of references

//References and Borrowing
//A reference is like a pointer in that it’s an address we can follow to access the data stored at that address
//hat data is owned by some other variable. Unlike a pointer.
//a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

// fn main(){
//     let s=String::from("hello");
//     let len=get_len(&s);
//     println!("length of {s} is {len}");
// }

// fn get_len(s:&String)->usize{
//     return s.len();
// }
// references allow you to refer to some value without taking ownership of it.

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//this works
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("{s}")
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
//  This code that attempts to create two mutable references to s will fail:

// fn main(){
//     let mut s=String::from("hello");
//     let s1=&mut s;
//     let s2=&mut s; //will not work
//     println!("{s},{s1},{s2}")

// }

// This error says that this code is invalid because we cannot borrow s as mutable more than once at a time.
//  The first mutable borrow is in s1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage
//  we tried to create another mutable reference in s2 that borrows the same data as s1.

// fn main(){
//     let mut s=String::from("hello");
//     let s1=&mut s;
//     println!("{s1}");
//     let s2=&mut s;
//     println!("{s2}") // this works since s1 is already used before creating s2

// }
//this is done to avoid race condition (Two or more pointers access the same data and changing it at the same time)
//after a mutable reference is used we can use another one

//you also cant create immutable reference after a mutable reference, till it is used.

//   let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//   let r2 = &mut s;


//cant do this since after one mutable ref, no mutable or immutable ref is allowed

// fn main(){
//   let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

// println!("{r1}, {r2}, and {r3}");

// }

//this works
// fn main(){
//     let mut s = String::from("hello");

//     let r1 = &s; 
//     let r2 = &s; 

//     println!("{r1},{r2}"); //its already being used

//     let r3=&mut s;

//     println!("{r3}");
// }

//code can have any number of immutable references that is not a problem
// fn main(){
//     let s = String::from("hello");

//     let r1 = &s; 
//     let r2 = &s; 
//     let r3 = &s; 
//     let r4 = &s; 

//     println!("{s},{r1},{r2},{r3},{r4}");
// }

// In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data,
// the compiler will ensure that the data will not go out of scope before the reference to the data does.
//recap ::At any given time, you can have either one mutable reference or any number of immutable references.

// fn main(){
//     let mut s=String::from("deva");
//     s.clear();// makes s an emoty string
//     println!("{s}");
// }

// String Slices
// A string slice is a reference to a contiguous sequence of the elements of a String, and it looks like this:

// fn main(){
//     let s=String::from("hello world");
//     let s1=&s[0..5];
//     let s2=&s[6..11];
//     let s3=&s[..5];
//     let s4=&s[6..];
//     println!("s1 :{s1}, s2: {s2}, s3: {s3}, s4: {s4}, s5:{s5}, s: {s}");
// }
// https://doc.rust-lang.org/book/img/trpl04-07.svg

//slices are also in arrays

// fn main(){
//     let arr=[1,2,3,4,5];
//     let a1=&arr[1..3];
//     assert_eq!(a1,&[2,3]);
//     println!("{a1:?}")
// }

// structs
// A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");
//     //you cant mark one specific field as mutable in a struct
//     //you can make the entire struct mutable
//     println!("{user1:?}");

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//         //means remaining field should be filled nu user1 properties
//     };

//     println!("{user2:?}")
// }

// Using Tuple Structs Without Named Fields to Create Different Types

// #[derive(Debug)]
// struct Color(i32, i32, i32);
// #[derive(Debug)]
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("{black:?}")
// }

//Ownership of Struct Data
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }
//this will throw an error because we need to specify lifetime for &str,
//ie if it goes out of scope, thr struct should also go out of scope

// #[derive(Debug)]
// struct Rect{
//     width:f32,
//     height:f32
// }

// fn main(){
//     let r1=Rect{
//         width:10.0,
//         height:10.0
//     };

//     println!("area is: {}",rect_area(r1));
// }

// fn rect_area(r:Rect)->f32{
//     return r.height*r.width;
// }

//Method Syntax

//Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.
//All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl

// struct Rect{
//     width:f32,
//     height:f32
// }

// impl Rect{
//     fn area(&self)->f32{
//         return self.width*self.height;
//     }

//     fn print_something(){
//         print!("something")
//     }
// }

// fn main(){
//         let r1=Rect{
//         width:10.0,
//         height:10.0
//     };

//     println!("area is: {}",r1.area()); //area is a method (it will work)
//     // println!("area is: {}",r1.print_something()); //this will not work (since it is a static function)
//     Rect::print_something();
// }

//Enums
//Enums allow you to define a type by enumerating its possible variants

// #[derive(Debug)]
// enum iptypes{
//     v4,
//     v6
// }

// #[derive(Debug)]
// struct ipaddress{
//     type_of:iptypes,
//     address:String
// }

// fn main(){
//     let user1=ipaddress{
//         type_of:iptypes::v4,
//         address:String::from("ibm/devops/admin/1")
//     };

//     println!("{user1:?}")
// }

//option enum
//match is used for pattern matching
//option enum is used when somethin can return a none

// fn main() {
//     let text = "hello world";
//     let target = 'o';

//     match find_char_index(text, target) {
//         Some(index) => println!("'{}' found at index {}", target, index),
//         None => println!("'{}' not found in \"{}\"", target, text),
//     }
// }

// fn find_char_index(s: &str, c: char) -> Option<usize> {
//     for (i, ch) in s.chars().enumerate() {
//         if ch == c {
//             return Some(i); // Found → return Some(index)
//         }
//     }
//     None // Not found → return None
// }

