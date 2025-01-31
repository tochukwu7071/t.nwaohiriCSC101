fn main() {
    
    let v = vec![ 101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variable owns the heap memory at anu given time
    let v2 = v;
    // here two varaibles owns heap value
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access , so it det ts a race condition
    //as two variables point to same heap

    println!("{:?}",v); 
}
