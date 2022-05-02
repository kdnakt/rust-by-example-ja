struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
// Valに対してimpl
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
// ジェネリック型`T`の場合のメソッドをGenValに対して実装
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
    println!("{:.2}, {}", x.value(), y.value());;
}
