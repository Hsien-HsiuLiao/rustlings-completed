// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
        1.. => Ok(PositiveNonzeroInteger(value as u64)),
        val if val < 0 => Err(CreationError::Negative),
        0 => Err(CreationError::Zero),
        _ => unreachable!()
            /*
            Another caveat is that rustc doesn’t know how to check exhaustive number matches. 
            So even if you think you’ve covered everything, you’re going to need a final catch-all match – like _ => unreachable!().
            https://users.rust-lang.org/t/help-matching-on-negative-integer/22127/2
            */
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
