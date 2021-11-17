use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

// This is a sample for aliasing; use Option<T> for practical use.
type Optional<T> = Result<T, ()>;

fn main() {
    // TryFrom
    let result = EvenNumber::try_from(8);
    assert_eq!(result, Ok(EvenNumber(8)));
    println!("{:?}", result);
    let result = EvenNumber::try_from(5);
    assert_eq!(result, Err(()));
    println!("{:?}", result);

    // TryInto
    let result: Optional<EvenNumber> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    println!("{:?}", result);
    let result: Optional<EvenNumber> = 5i32.try_into();
    assert_eq!(result, Err(()));
    println!("{:?}", result);
}
