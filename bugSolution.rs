fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable borrow
        let y = &mut x; 
        *y = 10; 
    }
    { // Another new scope for the second mutable borrow
        let z = &mut x;
        *z = 15;
    }
    println!("x = {}", x);
}
