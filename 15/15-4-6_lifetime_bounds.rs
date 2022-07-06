use std::fmt::Debug; // Trait to bound with.
                     // ライフタイムを紐付けるトレイト

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.
// `Ref`は`'a`というライフタイムを持つジェネリック型`T`に対する参照を持ち、
// `T`の値に対する *参照* は必ず`'a`よりも長生きでなくてはならない。
// さらに、`Ref`のライフタイムは`'a`を超えてはならない。

// A generic function which prints using the `Debug` trait.
// `Debug`トレイトを利用してプリントを行うジェネリック関数
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
// `Debug`を実装している`T`への参照を取る。`T`への *参照* は
// 必ず`'a`よりも長生きでなくてはならない。さらに、`'a`は
// 関数自体よりも長生きでなくてはならない。
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn print_ref2<'a, T>(t: &'a T) where
    T: Debug {
    println!("`print_ref2 - no where constraint`: t is {:?}", t);
}

fn print_ref3<'a, T>(t: & T) where
    T: Debug + 'a {
    println!("`print_ref3 - no param constraint`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print_ref2(&ref_x);
    print_ref3(&ref_x);
    print(ref_x);
}
