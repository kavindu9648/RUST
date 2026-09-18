/*Strings
Strings are used to store text.

You have already learned that you can use the &str type to create a string:*/

let greeting:&str="Hello";
println!("{}",greeting);

/*Create a String
You can create a String from a string literal using the to_string() method or the String::from() function:*/
//It is up to you which one to choose - both to_string() and String::from() are very common in Rust.
let text1="Hello World".to_string();
let text2=String::from("Hello World");

/*Change a String
Strings are mutable, so you can change them if they are declared with mut.

Use push_str() to add text to a string:*/
let mut greeting = String::from("Hello");
greeting.push_str(" World");
println!("{}", greeting); // Hello World

//Use push() to add one character:
let mut word=String::from("Hi");
word.push_str("!");
println!("{}",word);

/*Concatenate Strings
You can combine strings using the format! macro:
*/

let s1=String::from("Hello");
let s2=String::from("World");
let s3=String::from("Good");
let result=format!("{}{}{}",s1,s2,s3);
let result_two=s1+""+&s2+""+&s3;
println!("{}",result_two);
println!("{}",result);

/*String Length
You can use the .len() method to get the length of a string:*/
let name=String::from("Kavindu");
println!("Lenght:{}",name.len());
