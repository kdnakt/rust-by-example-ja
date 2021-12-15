fn binding_example1(age: u32) {
    match age {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        // `1 ... 12`の値を一挙に`match`させることができる。
        // しかしその場合、子供は正確には何歳?
        // マッチした値を`n`にバインディングすることで値を使用できる。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        // マッチしなかった場合の処理
        n             => println!("I'm an old person of age {:?}", n),
    }
}

#[derive(Debug)]
struct Movable { x: i32 }

#[derive(Clone, Copy, Debug)]
struct Copyable { x: i32 }

fn main() {
    println!("Tell me what type of person you are");
    binding_example1(8);   // Prints `I'm a child of age 8`.
    binding_example1(15);  // Prints `I'm a teen of age 15`.
    binding_example1(30);  // Prints `I'm an old person of age 30`.

    // Capturing by move
    let a = Movable { x: 100 };
    println!("a: {:p}", &a);
    println!("x: {:p}", &a.x);  // &a equals to &a.x, because `x` is the first member of `a`.
    match a {
        b @ Movable { x: y } => {
            println!("Moved a:      {:p}", &b);
            println!("Captured a.x: {:p}", &y); // &b not equals to &y, because `y` is newly captured variable.
            // println!("Original a: {:p}", &a);
            //                              ^^ value is borrowed after move
        }
    }

    // Capturing by copy
    let a = Copyable { x: 100 };
    println!("a: {:p}", &a);
    println!("x: {:p}", &a.x);
    match a {
        b @ Copyable { x: y } => {
            println!("Copied a:     {:p}", &b);
            println!("Captured a.x: {:p}", &y); // &b not equals to &y, because `y` is newly captured variable.
            println!("Original a:   {:p}", &a); // `a` can be borrowed.
        }
    }

    // Capturing by reference
    let a = Movable { x: 100 };
    println!("a: {:p}", &a);
    println!("x: {:p}", &a.x);
    match a {
        ref b @ Movable { x: ref y } => {
            println!("Referenced a:   {:p}", b);
            println!("           a.x: {:p}", y);    // &b equals to &y, because `y` captures a reference of a.x.
            println!("Original a:     {:p}", &a);
        }
    }
}