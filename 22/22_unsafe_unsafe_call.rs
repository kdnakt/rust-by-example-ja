use std::slice;

fn main() {
    let some_vector = vec![300, 2, 3, 4];

    let pointer: *const u32 = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        // FFI以外にも、スライスをばらしてポインタとして保存する
        // 型消去：どんな型でもいいからポインタを集めて処理したい場合もある
        let p2: *const u8 = pointer as *const u8;
        let my_slice: &[u8] = slice::from_raw_parts(p2, length);

        // assert_eq!(some_vector.as_slice(), my_slice);
        println!("{:?}", my_slice);
    }
}
