//In Rust, the type of a variable is decided by the value you give it. Rust looks at the value and automatically chooses the right type.

let num=5;
let double=5.99;
let letter='D';
let my_bool=true;
let text="Hello";

//it is possible to explicitly tell Rust what type a value should be - Wargaya kelinma sadahan kireeema
let num:i32=5;
let double:f64=5.99;
let letter:char='D'
let my_bool:bool=true;
let my_text:&str="Hello";

//Integer (i32)

let age:i32=25;
println!("Age is:{}",age);

//Floating Point (f64)
let price:f64=19.99;
println!("price is:{}",price);

//Characters (char)
let myGrade:char='K';
println!("My Char is:{}",myGrade);

//Strings (&str)
let name:&str="Kavindu Eranga";
println!("My name is:{}",name);
  
//Booleans (bool)
let is_bool:bool=true;
println!("This Is:{}",is_bool);

//Exercise
let name:&str="Kavindu Eranga";
let age:i32=28;
let age_floate:f64=28.72;
let charac:char='K';
let am_true:bool=true;
println!("My name is:{}",name);
println!("My age is:{}",age);
println!("My age_floate is:{}",age_floate);
println!("My Charac is:{}",charac);
println!("My am_true is:{}",am_true);


