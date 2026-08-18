 //Rust uses {} as a placeholder in println!() to show variable values.

 /*If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable)*/
   let mut x=5;
   println!("Before :{}",x);

   let x=10;
   println!("After:{}",x)

    let mut name="Kavindu Eranga";
    println!("My Name Is:{}",name);

    name="Eranga";
    println!("My Friends name \n is:{}",name);

    let village="Kegalle";
    print!("My Village \nIs:\n{}",village);


//You can use as many placeholders as you like:
    let name ="Kavindu Eranga";
    let age="28";

   print!("{} is {} years old.\n", name, age);
   print!("{} is {} years old.",age,name);