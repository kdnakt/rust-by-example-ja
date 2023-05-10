// Define this in a crate called `adder`.
// Assume that crate is called adder, will have to extern it in integration test.
// `adder`という名前のクレートの内部で、次の関数を定義する。
// インテグレーションテストでadderクレートをexternで宣言する。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}