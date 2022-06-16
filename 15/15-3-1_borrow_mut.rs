#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    // `&'static str`はread-onlyメモリ上の文字列への参照
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
// この関数はBook型へのリファレンスを取る。
fn borrow_book(book: &Book) {
    // book.year = 2022;
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
// この関数はミュータブルなBook型へのミュータブルなリファレンスを取り、
// `year`を2014へ変更する。
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named `immutabook`
    // `immutabook`という名のイミュータブルなBookを作成
    let immutabook = Book {
        // string literals have type `&'static str`
        // 「"」で囲まれた部分は`&'static str`型になる。
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    // `immutabook`のミュータブルなコピーを作成し、`mutabook`と名付ける
    let mut mutabook = immutabook;

    // Immutably borrow an immutable object
    // イミュータブルなオブジェクトをイミュータブルに借用する
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    // ミュータブルなオブジェクトをイミュータブルに借用する
    borrow_book(&mutabook);

    // Borrow a mutable object as mutable
    // ミュータブルなオブジェクトをミュータブルに借用する
    new_edition(&mut mutabook);

    borrow_book(&mutabook);

    // Error! Cannot borrow an immutable object as mutable
    // エラー！イミュータブルなオブジェクトをミュータブルに借用することはできない
    // new_edition(&mut immutabook);
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう
}
