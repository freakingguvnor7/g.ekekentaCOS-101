fn main() {
    // initialize a mutable tuple
    let mut mountain_heigths = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_heigths);

    // change 3rd and 4th element of a mutable tuple
    mountain_heigths.2 = "Lhotse";
    mountain_heigths.3 = 8516;


    println!("Changed tuple = {:?}", mountain_heigths);
}
