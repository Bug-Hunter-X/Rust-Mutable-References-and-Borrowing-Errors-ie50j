fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    let mut a = 10;
    {   // This block creates a new scope
        let b = &mut a;
        *b = 20;
    }
    println!("a = {}", a); // Output: a = 20

    let mut c = 30;
    // Correct way to handle multiple mutable references (often requiring different scopes) 
    {    
        let d = &mut c;
        *d += 1;
    }
    {        
        let e = &mut c; 
        *e += 5;  
    }
    println!("c = {}", c); // Output: c = 36
}