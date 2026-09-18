/*Variable Inside a Function
A variable created inside a function only exists inside that function:*/
fn myFunction() {
  let message = "Hello!";
  println!("{}", message);  // You can access the message variable here
}

myFunction();

println!("{}", message); // Error - you cannot access the message variable outside of the function

/*Variable Inside a Block
You can also create blocks inside other code, like in if statements or loops. Variables created in these blocks are only valid inside them.*/

let score = 80;

if score > 50 {
  let result = "Pass";
  println!("Result: {}", result);
}

println!("Result: {}", result); // Error: result is out of scope here

/*Variables in the Same Scope
In Rust, you can declare a new variable with the same name in the same scope using let. This is called shadowing:*/

let x = 5;
let x = 10;

println!("x is: {}", x); // prints 10

/*The second x replaces the first one. The value 5 is no longer accessible after the second declaration.

This is different from languages that disallow reusing variable names. In Rust, it's a feature used to transform or update values safely.

You can also reuse a variable name inside a new block:*/


let x = 5;

{
  let x = 10;
  println!("Inside block: {}", x);
}

println!("Outside block: {}", x);