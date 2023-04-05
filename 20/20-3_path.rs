use std::path::Path;

fn main() {
    // Create a `Path` from an `&'static str`
    // `&'static str`から`Path`を作成
    let path = Path::new(".");

    // The `display` method returns a `Display`able structure
    // `display`メソッドは`Display`可能な構造体を返す。
    let display = path.display();
    println!("path: {}", display);

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    // `join`はOS固有のセパレータによってバイトのコンテナ型であるパス
    // を結合し、`PathBuf`を返す。
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`
    new_path.set_file_name("package.tgz");

    // Convert the `PathBuf` into a string slice
    // `PathBuf`を文字列のスライスに変換する。
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    };
    println!("new_path is {}", new_path.display());
}
