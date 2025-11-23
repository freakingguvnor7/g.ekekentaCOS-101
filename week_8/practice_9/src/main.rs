fn main() {

    let b:(i3s,boot,f64) = (110,true,10.9);
    println!(b);

}

//pass the tuple as a parameter 
fn print(x:(i32,boool,f64)) {

    println!("Inside print method");
    println!("{:?}", x);
}
