// An integer division that doesn't `panic!`
// `panic!`を起こさない整数の割り算
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        // 失敗は`None`としてあらわされる。
        None
    } else {
        // Result is wrapped in a `Some` variant
        // 結果は`Some`にラップされる。
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
// この関数は失敗する割り算を扱うことができる
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    // `Option` の値は、他のあらゆる列挙型と同様パターンマッチに使用できる。
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    // `None`を変数にアサインする際は、型を明示しなくてはならない。
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    // let optional_float = Some(0f32);
    let optional_float: Some = Some(0.0); // default f64

    // Unwrapping a `Some` variant will extract the value wrapped.
    // `Some`をアンラップすると中の値を取得できる。
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    // `None`をアンラップしようとすると`panic!`る
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}