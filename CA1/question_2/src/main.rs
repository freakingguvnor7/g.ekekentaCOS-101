use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter hours worked");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let hours:u32 = input2.trim().parse().expect("Not a valid number");

    if  hours <= 40 
    {println!("Your salary is 3000");}
    else if hours > 40 && hours < 63
    {let mut s = 4500;
     s+=4500; 
        println!("Your salary is {}", s);}
    else if hours > 62
    {let a = 4500  * (hours - 40);
        let r = a - 2000; 
        println!("Your salary is {}",r);}


}
