fn main() {
    let mut x = 100;
    let y = &mut x;
    println!("{}", y);
    *y += 100;
    println!("by dereferencing y and getting x instead of the address to x, 
    we can update x which in turn updates y to {}", y);
    println!("{}", *y == x);

    let z = &mut x;
    z += 1000;
    assert_eq!(x, 1200);
}
