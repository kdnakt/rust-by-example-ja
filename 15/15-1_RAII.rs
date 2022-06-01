fn create_box() {
    // 整数をヒープ上に確保
    let box1 = Box::new(3i32);
    println!("Address of box1 on stack: {:p}", &box1);
    println!("Address of *box1 on heap: {:p}", box1);
    // `box1`はここで破棄され、メモリは解放される。
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    // 整数をヒープ上に確保
    let box2 = Box::new(5i32);
    println!("Address of box2 on stack: {:p}", &box2);
    println!("Address of *box2 on heap: {:p}", box2);

    // ネストしたスコープ
    {
        // 整数をヒープ上に確保
        let box3 = Box::new(4i32);
        println!("Address of box3 on stack: {:p}", &box3);
        println!("Address of *box3 on heap: {:p}", box3);
        // `box3`はここで破棄され、メモリは解放される。
    }

    // お遊びで大量のボックスを作る。
    // もちろん手動で開放する必要はないよ！
    for _ in 0u32..2 {
        create_box();
    }

    let to_drop1 = ToDrop;
    println!("Address of to_drop1 on stack: {:p}", &to_drop1);
    let to_drop2 = Box::new(ToDrop);
    println!("Address of to_drop2 on stack: {:p}", &to_drop2);
    // ToDropはフィールドを持たない構造体なので、実体をメモリ上に展開する必要がない。
    // それゆえ、Box::newは実際にはメモリ確保を行わず、仮のアドレス(手元では0x1)を使う。
    println!("Address of *to_drop2 on heap: {:p}    // Oops!", to_drop2);

    // `box2`はここで破棄され、メモリは解放される。
    // `to_drop`のdropはここで呼ばれ、リソースの解放処理が走る。
}

