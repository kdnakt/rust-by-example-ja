fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // エラー! `_mutable_integer`はこのスコープではフリーズしている。
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line
        // FIXME ^ この行をコメントアウトしましょう。

        // `_mutable_integer` goes out of scope
        // `_mutable_integer`はスコープを抜ける
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    // OK! `_mutable_integer`はこのスコープではフリーズしていない。
    _mutable_integer = 3;
}
