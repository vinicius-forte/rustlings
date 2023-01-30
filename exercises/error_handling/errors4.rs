// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

use std::cmp::Ordering;

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value.cmp(&0) {
            Ordering::Greater => Ok(PositiveNonzeroInteger(value as u64)),
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
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
