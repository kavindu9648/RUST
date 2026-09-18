fn first_function(){
  println!("Hello");
}
first_function();

//Functions with Parameters
//You can send information into a function using parameters. Parameters are written inside the parentheses ().

fn greet(name:&str){
    println!("Hello,{}",name);
}
greet("Kavindu");

/*Functions with Return Values
A function can also return a value.

Use the  -> symbol in the function header to show what type of value will be returned.

Inside the function, use the return keyword to send the value back:*/

fn add (a:i32,b:i32)->i32{
    return a+b;
}
let sum = add(3,4);
println!("Sum is:{}",sum);

/*This function adds two numbers and returns the result.

In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon:*/
fn add(a: i32, b: i32) -> i32 {
  a + b
}

let sum = add(3, 4);
println!("Sum is: {}", sum);
