use std::num::ParseIntError;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
// `ParseIntError`を`Err`の型として持つ全ての`Result`のジェネリックエイリアス
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
// 上で定義したエイリアス（この場所特有の`Result`型）を使用
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn multiplyi64(first_number_str: &str, second_number_str: &str) -> AliasedResult<i64> {
    first_number_str.parse::<i64>().and_then(|first_number| {
        second_number_str.parse::<i64>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
// もう一度使用。エイリアスによって再度明記する必要性がない。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn printi64(result: AliasedResult<i64>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
    print(multiply("12345678901", "3332"));
    printi64(multiplyi64("10", "2"));
    printi64(multiplyi64("12345678901", "3332"));
    printi64(multiplyi64("t", "2"));
}