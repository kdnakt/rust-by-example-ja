#![allow(unused)]
​
//#[derive(Debug)]
struct Deep(Structure);
​
fn main() {
    println!("{}",Structure(28));
    println!("Now {} will print!", Deep(Structure(7)));
    
}
​
// Import (via `use`) the `fmt` module to make it available.
// （`use`を使用し、）`fmt`モジュールをインポートします。
use std::fmt;
​
// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
// `fmt::Display`を実装するための構造体を定義します。
// これは`Structure`という名前に紐付けられた、`i32`を含むタプルです。
struct Structure(i32);
​
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// `{}` というマーカーを使用するためには、
// この型専用の`fmt::Display`というトレイトが実装されていなくてはなりません。
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 必ず、第一の要素が出力されるようにしています。
        // `f`は`fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.0)
    }
}
​
impl fmt::Display for Deep {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 必ず、第一の要素が出力されるようにしています。
        // `f`は`fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.0)
    }
}
