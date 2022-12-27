use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
// 自前のエラー型の定義。エラーハンドリングのケースの応じてカスタマイズされる。
// ここで新たなエラーを書くことができ、元のエラーの実装に処理を委譲したり、
// その手前で何らかの処理を挟むことができます。
#[derive(Debug, Clone)]
struct DoubleError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
// エラーの生成は、それがどのように表示されるかとは別物です。
// そのため、エラーの表示スタイルに関する複雑なロジックを煩雑になるなどと気にする必要はありません。
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
// エラーに関する余分な情報を持たせていないことに注意してください。
// どの文字列がパースに失敗したかなどを出力することは、
// その情報を保持させるようにエラーの定義を修正しない限りできません。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        // 基本となるエラー、原因は記録されていない。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let s = vec.first()
        // Change the error to our new type.
        // エラーを新たな型に変更する。
        .ok_or(DoubleError)?;
        s.parse::<i32>()
                // Update to the new error type here also.
                // ここでも新たなエラー型に更新する。
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
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
