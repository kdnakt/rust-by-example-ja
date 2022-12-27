// `find_min!` will calculate the minimum of any number of arguments.
// `find_min!`は引数として与えられた数字の中の最低の値を計算する。
macro_rules! find_min {
    // Base case:
    // 基本となるケース
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    // `$x`に少なくとも1つの`$y`が続く場合
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        // `find_min!`を残りの`$y`に対して再帰的に適用
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
