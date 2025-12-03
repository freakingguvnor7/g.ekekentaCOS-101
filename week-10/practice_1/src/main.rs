fn main() {
    
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variables owns the heap at any given time
    let v2 = v;
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access ,so it dtects a race condition
    //as two variaables point to same heap

    println!("{:?}",v2);
}
