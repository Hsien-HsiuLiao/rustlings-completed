// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)


fn main() {
    let mut x = 100;
    let y = &mut x;
    println!("{}", y);
    *y += 100;
    println!("by dereferencing y and getting x instead of the address to x, 
    we can update x which in turn updates y to {}", y);
    println!("{}", *y == x);

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
