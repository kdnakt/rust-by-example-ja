// An attribute to hide warnings for unused code.
// 使用されていないコードよる警告を隠すアトリビュート
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // `use`することで絶対名でなくとも使用可能になる。
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // [memo] 元のenumと値の順番が異なっても問題ない

    // use Status::{Poor, Rich};
    // [memo] 今回の場合は、crateがなくても動作する

    // use Status::{Poor, Rich, Normal};
    // [memo] 元のenumに存在しない値は設定できない、利用する場合は定義すること
    // error[E0432]: unresolved import `Status::Normal`
    //      use Status::{Poor, Rich, Normal};
    //                               ^^^^^^ no `Normal` in `Status`
    // For more information about this error, try `rustc --explain E0432`.
    // error: could not compile `playground` due to previous error

    // Automatically `use` each name inside `Work`.
    // `Work`の中の名前をすべて`use`する
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    // `use`しているため、`Status::Poor`と書いていることに等しい
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    // `Work::Civilian`に等しい
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        // `use`しているのでスコープを明示していない
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
        _ => println!("normal.")
    }

    match work {
        // Note again the lack of scoping.
        // こちらも同じ
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}

// Result
// The poor have no money...
// Civilians work!