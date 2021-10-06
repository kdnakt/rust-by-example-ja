// Globals are declared outside all other scopes.
// グローバル変数はあらゆるスコープの外で宣言します
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    // 関数内から定数を参照
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 基本的にstatic変数は変えちゃ駄目。変えられる方法もあるが面倒。単純に以下の様に代入するとエラー
    // なので基本的にはconstを使う。static使うのはほぼ無い。
    // lazy_staticとかonce_cellとか。上級者向け。
    // 参考：https://zenn.dev/frozenlib/articles/lazy_static_to_once_cell
    // LANGUAGE = "it's rust.";
    
    // Access constant in the main thread
    // main 関数の中から定数を参照
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // エラー!`const`は変更できません。
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう
}