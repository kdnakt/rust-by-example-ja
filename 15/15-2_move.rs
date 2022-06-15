// This function takes ownership of the heap allocated memory
// この関数はヒープメモリ上の資源の所有権を取る。
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
    // `c`は破棄されメモリは開放される。
}

fn main() {
    // _Stack_ allocated integer
    // _スタック_上に置かれた整数
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    // `x`を`y`に *コピー* する。元の値が移動するわけではない。
    let y = x;

    // Both values can be independently used
    // 両方の値はそれぞれ独立に使うことができる。
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    // `a`は_ヒープ_上の整数へのポインタ
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    // `a`を`b`に *ムーブ* する。
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    // すなわち、`a`の指すメモリ上の番地が`b`にコピーされるため
    // いずれもヒープ上の同じ値を指すポインタとなる。しかし所有権は`b`にある。
    
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // エラー! `a`は所有権を持たないため、ヒープ上のデータにアクセスできない。
    //println!("a contains: {:p}", &a);
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにここをアンコメントしてみましょう。

    // `b`は所有権があるので、アクセスできる。
    println!("b contains: {:p}", &b);

    // This function takes ownership of the heap allocated memory from `b`
    // この関数はヒープメモリ上の所有権を`b`から取る。
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // この時点でヒープメモリ上の資源は開放されているので、次の操作は
    // 解放済みメモリをデリファレンスすることになる。しかしそれはコンパイラが許さない。
    // エラー! 上述の理由より
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
    // TODO ^ 試しにここをアンコメントしてみましょう。
}
