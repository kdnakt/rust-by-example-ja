// use std::convert;
use std::error;
use std::fmt;

// Change the alias to `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    // ? 演算子は Err(From::from(err)) をするので、EmptyVec -> Box<dyn error::Error>の変換をしてくれる。
    let first = vec.first().ok_or(EmptyVec)?;
    // intoを使ってもOK。ただし ? 演算子は Err(From::from(err)) をするので、この場合は宛先の型を補う必要がある。
    // let first = vec.first().ok_or_else(|| {
    //     let e: Box<dyn error::Error> = EmptyVec.into();
    //     e
    //     // もしくは, <EmptyVec as Into<T>>::into(EmptyVec)
    // })?;
    // fromを使ってもOK。ただし ? によるfromがなくなるわけではないので、intoと同じ問題がある。
    // let first = vec.first().ok_or_else(|| Box::<dyn error::Error>::from(EmptyVec))?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

