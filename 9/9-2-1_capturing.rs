fn main() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    // `color`をプリントするためのクロージャ。
    // これは`color`を借用(`&`)し、その借用とクロージャを`print`
    // という名の変数に保持する。
    // 借用は`print`がスコープから出るまで続く。
    // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    // 借用を行ったクロージャをコールする。
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;
    // print()

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    // `count`をインクリメントするためのクロージャ。`count`と`&mut count`
    // の両方を取ることができるが、後者のほうが制限が少ないため、
    // （訳注: `count`だと`&mut count`と違い、一度しか呼ぶことができない。）
    // そちらを取る。直後に`count`を借用する。
    //
    // `inc`には`mut`をつける必要がある。なぜならミュータブルな型が
    // 中で使用されているからである。ミュータブルなクロージャは呼ぶたびに
    // 内部変数を変更する。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    // クロージャを実行
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;


    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    // `mem::drop`は`T`（ジェネリック型）を取る必要があるため、このクロージャは
    // 参照ではなく値を取る。その場合、もしもコピー可能な値ならば、
    // 元の値はそのままでコピーのみを取る。不可能ならば値そのものを移動させる。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    // `consume`は変数を消費（開放）するため、一度しか呼び出すことができない。
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
    // ^ TODO: この行のコメントアウトを解除しましょう。


    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // alternatively:
    let contains = |needle| {
        let haystack = vec![1, 2, 3];
        haystack.contains(needle)
    };
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}
