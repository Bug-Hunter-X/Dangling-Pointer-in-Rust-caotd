fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // The following line causes undefined behavior because vec is dropped, freeing the memory
    // pointed to by ptr.  Reading from ptr after this point will lead to memory corruption.
    drop(vec);

    println!("The first element is: {}", unsafe { *ptr });
}