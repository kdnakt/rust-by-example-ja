/// First line is a short summary describing function.
/// 最初の行には関数の機能の短い要約を書きます。
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
/// 以降で詳細なドキュメンテーションを記述します。コードブロックは三重のバッククォートで始まり、
/// 暗黙的に`fn main()`と`extern crate <クレート名>`で囲われます。
/// `doccomments`クレートをテストしたいときには、次のように記述します。
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
/// 一般的に、ドキュメンテーションコメントは「実行例」「パニック」「失敗」という章から成る。
///
/// The next function divides two numbers.
/// 次の関数は除算を実行する。
///
/// # Examples
/// # 実行例
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
/// # パニック
///
/// The function panics if the second argument is zero.
/// 第2引数がゼロであればパニックする。
///
/// ```rust,should_panic
/// // panics on division by zero
/// // ゼロで除算するとパニックする
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}