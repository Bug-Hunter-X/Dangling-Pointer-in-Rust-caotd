fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access the first element
    println!("The first element is: {}", vec[0]);
    
    //Alternatively, borrow the vector and print the first element.
    //This avoid taking the pointer and therefore the problem of dangling pointer
    let first_element = &vec[0];
    println!("The first element is: {}", first_element);
} 