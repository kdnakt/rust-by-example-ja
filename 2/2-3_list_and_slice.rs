use std::mem;

// This function borrows a slice
// この関数はスライスを借用する
// fn analyze_slice(slice: &mut[i32]) { // 関数内で可変にする場合mut宣言が必要（呼び出し側も必要）
fn analyze_slice(slice: &[i32]) {
    // slice[0] = 12; // mut宣言をしておくと書き込める
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    // 固定長の配列（型シグネチャは冗長なので、なくても可）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // let mut xs: [i32; 5] = [1, 2, 3, 4, 5]; // 書き込める変数宣言
    // xs[0] = 11; // mut宣言しておくと上書きできる

    // All elements can be initialized to the same value
    // すべての要素を0にする場合
    let ys: [i32; 500] = [0; 500];
    // let mut ys: [i32; 5] = [10, 20, 30, 40, 50]; // 同上

    // Indexing starts at 0
    // インデックスは０から
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    // `len`は配列のサイズを返す。
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    // 配列はスタック上に置かれる
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    // 配列は自動的にスライスとして借用される。
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    // analyze_slice(&mut xs); // 呼び出し先で書き込めるようにするためにはmut渡しする

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    // analyze_slice(&mut ys[1 .. 40]); // 同上。配列サイズオーバーチェックは実行時に行われる

    // Out of bound indexing causes compile error
    // インデックスの範囲が配列のサイズを超えた場合パニックする
    println!("{}", xs[4]); // 実行出来るようにするため5から4に変更。コンパイル時にサイズチェックが行われる
}