struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
// 以下の関数はトレイト境界を設けているが、そのトレイトが空である
// か否かとは関係がない。
fn color<T: Red>(_: &T)   -> &'static str { "red" }
fn color<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    // 訳注: 以下は全て鳥の名前
    // 猩々紅冠鳥
    let cardinal = Cardinal;
    // アオカケス
    let blue_jay = BlueJay;
    // 七面鳥
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    // トレイト境界のため、`red`は`blue_jay`に対しては使用できない。
    // `blue`と`Cardinal`も同様、
    println!("A cardinal is {}", color(&cardinal));
    println!("A blue jay is {}", color(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
    // ^ TODO: この行をアンコメントしてみましょう。
}
