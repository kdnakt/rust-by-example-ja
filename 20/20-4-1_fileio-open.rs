use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    // 目的ファイルに対する`Path`を作成
    let path = Path::new("20-4-1_hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // pathを読み込み専用モードで開く。これは`io::Result<File>`を返す。
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // file を介さずにファイルの内容を読み込む
    let file_contents: String = fs::read_to_string(path)
        .expect("couldn't open {display}");
    println!("file_content: {file_contents}");

    // `file` goes out of scope, and the "hello.txt" file gets closed
    // `file`がスコープから抜け、"hello.txt"が閉じられる。
}

