let a = String::from("Hello");
let b = &a;

println!("a={}",a);
println!("b={}",b);

let mut name=String::from("Kavindu");
let name_ref=&mut name;
name_ref.push_str("Eranga");
println!("{}",name_ref);