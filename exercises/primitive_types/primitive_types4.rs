// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// I AM NOT DONE 

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..=3];

    println!(r#"{:?}"#, nice_slice);

    assert_eq!([2, 3, 4], nice_slice)
}
/*

If you're curious why the first argument of `assert_eq!` does not
have an ampersand for a reference since the second argument is a
reference, take a look at the Deref coercions section of the book:
https://doc.rust-lang.org/book/ch15-02-deref.html
 */