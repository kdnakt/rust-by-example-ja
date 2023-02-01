fn main() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    // （以下の例では型を明示していますが、これらは必須ではありません。）
    // read only memory上に割り当てられた文字列への参照
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    // 単語を逆順にイテレートする。新しい文字列の割り当ては起こらない。
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    // 文字をベクトルにコピー。ソートして重複を除去
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("chars: {:?}", chars);

    // Create an empty and growable `String`
    // 中身が空で、伸長可能な`String`を作成
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        // 文字を文字列の末端に挿入
        string.push(c);
        // Insert a string at the end of string
        // 文字列を文字列の末端に挿入
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    // 文字列のトリミング（特定文字種の除去）はオリジナルの文字列のスライスを
    // 返すので、新規のメモリ割り当ては発生しない。
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    // 文字列をヒープに割り当てる。
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    // 新しくメモリを確保し、変更を加えた文字列をそこに割り当てる。
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}