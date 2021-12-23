/* これは冗長な書き方：if letで書き直す
// Make `optional` of type `Option<i32>`
// `optional`という変数の型を`Option<i32>`に指定
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
        // ^ `i`をoption型からデストラクトするためだけに
        // インデントが一つ増えてしまっている。
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
    // ^ `match`は全ての型に対して網羅的でなくてはならないので必要。
    // 冗長に見えませんか？
};
*/
fn main() {
    // All have type `Option<i32>`
    // 全て`Option<i32>`型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    // `if let`文は以下と同じ意味.
    //
    // もしletがnumberをデストラクトした結果が`Some(i)`になるならば
    // ブロック内(`{}`)を実行する。
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    // デストラクトした結果が`Some()`にならない場合の処理を明示したい場合、
    // `else`を使用する。
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        // デストラクト失敗の場合。このブロック内を実行
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    // デストラクト失敗時の処理を更に分岐させることもできる
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    // デストラクト失敗。`else if`を評価し、処理をさらに分岐させる。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        // 今回は`else if`の評価がfalseなので、このブロック内がデフォルト
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // 式として代入できる
    let foo = if let Some(i) = number {
        i
    } else {
        0
    };
    println!("{:?}", foo); // 7

    // unwrap_or()を使うとより簡潔に書ける
    let foo = number.unwrap_or(0);
    println!("{:?}", foo); // 7

    let bar = letter.unwrap_or(0);
    println!("{:?}", bar); // 0
}

