//1..6 means from 1 up to (but not including) 6.
for i in 1..6{
    println!("i is:{}",i);

for i in 1..=6{
    println!("i is:{}",i);
}
for i in 0..=10{
    if i==3{
        continue;//skip 3
}
    if i==5{
        break;

    }
    println!("i is:{}",i);
}

}
