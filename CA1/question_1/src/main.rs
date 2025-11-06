use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter temperature in celsius: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let temp:f32 =  input.trim().parse().expect("Not a valid number");

    let f:f32 = (9.0/5.0) * temp + 32.0;
    let k:f32 = temp + 273.15;

    println!("Temperature in celsius {}", temp);
    println!("Temperature in Fahrenheit {}", f);
    println!("Temperature in Kelvin {}", k);

    if temp < 0.0
    { println!("Freezing point");}

    else if temp >= 0.0 && temp <=30.0
    { println!("This is found in the normal range");}

    else if temp > 30.0
    {println!("This is a hot temperature");}

    else if temp < -273.0
    { println!("Invalid input");}


}
