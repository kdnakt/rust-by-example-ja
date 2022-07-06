// One input reference with lifetime `'a` which must live
// at least as long as the function.
// 引数として`'a`のライフタイムで参照を一つ取る。最低でもこの関数
// と同じだけの長さでなくてはならない。
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
// ミュータブルな参照でも同様
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
// 異なるライフタイムを持つ複数の引数がある場合。
// ここでは1種類のライフタイムでも問題はないが、より複雑なケースでは
// 異なるライフタイムが必要になる場合がある。
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
// 受け取った参照をそのまま返すことに問題はないが、適切なライフタイム
// でなくてはならない。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.
// `'a`は関数より長くなくてはならないため上の関数は正しくない。
// ここでは、`&String::from("foo")`は`String`のデータとそれへの参照を作り出す。
// その後データはスコープを抜けるとともに破棄される。そのため、
// 不適切なデータに対する参照を返すことになってしまう。

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
