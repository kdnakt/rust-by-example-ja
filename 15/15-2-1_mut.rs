// この関数はヒープメモリ上の資源の所有権を取ると同時にミュータビリティを変更する
fn to_mutable_box(mut c: Box<u32>) {
    *c = 3;
    println!("mutable box that contains {}", c);
}


fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    // ミュータビリティエラー
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    // boxを *ムーブ* する、同時に所有権とミュータビリティを変更する。
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    // boxの内容を変更
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
    
    // 関数
    let immutable_box2 = Box::new(5u32);
    to_mutable_box(immutable_box2);
}
