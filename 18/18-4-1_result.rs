use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    // `unwrap()`で数字を取り出してみましょう。痛い目を見るでしょうか？
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn safe_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // matchで数字を取り出します。安全にはなりましたが、かなり複雑な処理に見えます。
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e)            => return Err(e),
        },
        Err(e)           => return Err(e),
    }
}

fn main() -> Result<(), ParseIntError> {    // mainからは Result<(), E> を返すこともできます。
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);   // double is 20
    // let tt = multiply("t", "2");     // panic!
    // println!("double is {}", tt);

    let twenty = safe_multiply("10", "2");
    println!("double is {:?}", twenty); // double is Ok(20)
    let tt = safe_multiply("t", "2");
    println!("double is {:?}", tt);     // double is Err(ParseIntError { kind: InvalidDigit })
    Ok(())
}
