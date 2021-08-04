#![allow(unused)]
fn main() {
// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
// この構造体は`fmt::Display`、`fmt::Debug`のいずれによっても
// プリントすることができません。
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// `derive`アトリビュートは、
// この構造体を`fmt::Debug`でプリントするための実装を自動で提供します。
#[derive(Debug)]
struct DebugPrintable(i32);
}
