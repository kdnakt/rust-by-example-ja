fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
                                      // エラー１の生成
    2 * first.parse::<i32>().unwrap() // Generate error 2
                                      // エラー２の生成
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty
    // エラー１：入力が空ベクトル

    println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
    // エラー２：要素が数字としてパースできない
}
