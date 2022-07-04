// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        // 10の実体はどこに格納されるのだろう？
        // -> 恐らく静的定数が置かれる領域
        Self { x: &10 }
    }
}

struct A(i32);

fn f() -> &'static A { &A(100) }

fn g<'a>() -> &'a A { &A(100) }

fn h<'a>() -> &'a A { &A(200) }

const CONSTANT_A: A = A(100);    // Global constant of A(100)

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
    let p_f = f() as *const A;
    assert!(p_f == &CONSTANT_A); // f() and &a point to the same address.
    assert!(p_f == g());         // f() and g() point to the same address.
    assert!(p_f != h());         // f() and g() don't point to the same address.
}
