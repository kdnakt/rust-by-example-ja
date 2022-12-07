use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`.
// エイリアスを`Box<error::Error>`に変更する。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Boxに変換
        // .ok_or(EmptyVec.into())  // これだと余計な動的メモリ確保が起こる
        .and_then(|s| {
            s.parse::<i32>()
                .map(|i| 2 * i)
                .map_err(|e| e.into()) // Converts to Box
                                       // Boxに変換
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
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
