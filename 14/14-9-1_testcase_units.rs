use std::ops::Add;
use std::marker::PhantomData;

/// Create void enumerations to define unit types.
/// 単位を定義するため、空の列挙型を作成。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` is a type with phantom type parameter `Unit`,
/// and is not generic over the length type (that is `f64`).
/// `Length`は`Unit`という幽霊型パラメータを持つ型
///
/// `f64` already implements the `Clone` and `Copy` traits.
/// `f64`ははじめから`Clone`、`Copy`トレイトを持っている。
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64);

/// The `Add` trait defines the behavior of the `+` operator.
/// `Add`トレイトは加算演算子(`+`)の挙動を定義する。
impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    // add() returns a new `Length` struct containing the sum.
    // add()は`Length`の新しいインスタンスを返す。
    // Lengthの中の値は合計値になっている。
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for `f64`.
        // ここでの`+`は`f64`の`Add`実装を呼び出す。
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Specifies `one_foot` to have phantom type parameter `Inch`.
    // `one_foot`が幽霊型`Inch`を持つように明示する。
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` has phantom type parameter `Mm`.
    // `one_meter`が幽霊型`Mm`を持つように明示する。
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    // 以下の`+`は上で定義した`Length<Unit>`用の`add()`メソッドを呼び出す。
    //
    // Since `Length` implements `Copy`, `add()` does not consume
    // `one_foot` and `one_meter` but copies them into `self` and `rhs`.
    // `Length`は`Copy`トレイトを持っているため、`add()`は`one_foot`及び`one_meter`
    // を消費する代わりにそのコピーを作り、`self`、`rhs`として扱う。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    // 加算が問題なく実行されていることを確認
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Nonsensical operations fail as they should:
    // Compile-time Error: type mismatch.
    // 異なる単位間の加算は意味を成さないので、
    // 以下はきちんとコンパイルエラーになる。
    // コンパイルエラー: タイプミスマッチ
    // let one_feter = one_foot + one_meter;
}
