/// Using hidden `try_main` in doc tests.
/// ドキュメンテーションテストで、`try_main`を隠して使う。
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compilable!
/// # // 行頭に `#` を置くと行が隠されるが、コンパイルには成功する。
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// #                                       // ドキュメントの本体を囲う行
/// let res = doccomments::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// #        // try_mainから値を返す
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #             // unwrap()を実行するmain
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// #                         // try_mainを呼びunwrapすると、エラーの場合にパニックする
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}