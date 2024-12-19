fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    // *z += 1; // This would cause a compile-time error because z is immutable
    println!("x = {}", x); // Output: x = 6

    let mut a = 10;
    {   // This block creates a new scope
        let b = &mut a;
        *b = 20;
    }
    println!("a = {}", a); // Output: a = 20

    let mut c = 30;
    let d = &mut c;
    let e = &mut c;
    // *d += 1; // Compile-time error: Cannot borrow `c` as mutable more than once at a time
}