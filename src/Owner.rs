let mut boy=String::from("Hello");
let b=boy;
 //println!("{}",a);Error: a no longer owns the value
println!("{}",b);

let a = 5;
let b = a;
println!("a = {}", a);  // Works
println!("b = {}", b);  // Works

let a = String::from("Hello");
let b = a.clone(); // Now both have the same value

println!("a = {}", a);  // Works
println!("b = {}", b);  // Works

let a=String::from("Hello");
let b=&a;