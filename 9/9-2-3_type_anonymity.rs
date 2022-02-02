#![feature(unboxed_closures)]
#![feature(fn_traits)]
// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
// `F`は`Fn`を実装していなくてはならず、`Fn`は引数と返り値を持たない。
// `print`は文字をプリントするだけのクロージャなので、これが正しい。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

// 無名ではない構造体
struct PrintFn {
    x: i32
}

// FnOnce, FnMut, Fnは同時に宣言する必要がある
// それぞれに対応するcallを宣言する
impl FnOnce<()> for PrintFn {
    type Output = ();
    extern "rust-call" fn call_once(self, args: ()) { println!("{}", self.x) }
}

impl FnMut<()> for PrintFn {
    extern "rust-call" fn call_mut(&mut self, args: ()) { println!("{}", self.x) }
}

impl Fn<()> for PrintFn {
    extern "rust-call" fn call(&self, args: ()) { println!("{}", self.x) }
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    // `x`を無名の構造体に入れ、それに対し`Fn`を実装する。
    // （訳注: ここでは`Fn`は`fn Fn(&self) -> {println!("{}", &self)}`）
    // その構造体を`print`にアサインする。
    let print = || println!("{}", x);
    apply(print);

    // 無名ではない構造体`PrintFn`を宣言し、`printFn`にアサインする。
    let printFn = PrintFn{x};
    apply(printFn);   
}
