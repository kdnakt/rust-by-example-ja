// A concrete type `A`.
// `A`という具象型
#[derive(Debug)]
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
// `Single`という型を定義する際に`A`を使用しているが、その最初の使用よりも
// 先に`<A>`がないため、また、`A`自身も具象型であるため、`Single`は具象型となる。
#[derive(Debug)]
struct Single(A);
//            ^ Here is `Single`s first use of the type `A`.
//            ^ Singleによる`A`の一番最初の使用はここ

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
// ここでは`<T>`が一番初めの`T`の使用よりも先に来ている。よって`SingleGen`はジェネリック型
// となる。なぜならば型パラメータ`T`がジェネリックだからである。`T`はどんな型にもなりえるため、
// 上で定義した`A`を受け取ることもできる。
#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`.
    // `Single`は具象型で、`A`のみを受け取る。
    let _s = Single(A);
    println!("{:?}", _s);

    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`.
    // Here, `SingleGen` has a type parameter explicitly specified.
    // `_char`という名の変数を生成する。これは`SingleGen<char>`
    // という型で、値は`SingleGen('a')`となる。ここでは、`SingleGen`には明示的な型パラメータ
    // が与えられている。
    let _char: SingleGen<char> = SingleGen('a');
    println!("{:?}", _char);

    // `SingleGen` can also have a type parameter implicitly specified:
    // `SingleGen`型の変数には明示的に型パラメータを与えなくてもよい。
    let _t    = SingleGen(A); // Uses `A` defined at the top.
                              // 上で定義した`A`を使用
    println!("{:?}", _t);
    let _i32  = SingleGen(6); // Uses `i32`.
                              // `i32`を使用
    println!("{:?}", _i32);
    let _char = SingleGen('a'); // Uses `char`.
                                // `char`を使用
    println!("{:?}", _char);
}
