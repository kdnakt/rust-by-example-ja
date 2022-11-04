// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
// 大人は経験豊富なので、大体どんな飲み物にも対処できます。
// あらゆる飲み物は`match`を用いて手動で処理されます。
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
// 他の人たちは甘い飲み物を飲む前に`panic`します。
// 全ての飲み物は`unwrap`を使って暗黙的に処理されます。
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    // `unwrap`を使用すると値が`None`だった際に`panic`を返します。
    let inside = drink.unwrap_or_else(|| drink_tea());
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn drink_tea() -> &'static str {
    println!("tea!!!!");
    "tea"
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
