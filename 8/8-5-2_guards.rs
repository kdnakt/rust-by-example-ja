fn guard_example1(pair: (i32, i32)) {
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        //     ^ `if`とそれに続く条件式がガードです。
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

#[derive(Clone, Copy, Debug)]
struct A {
    x: i32,
}

fn main() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`
    // TODO ^ `pair`の値を変更してみましょう。
    guard_example1(pair);

    // When a expression matches multiple guards,
    // the expression is captured by the topmost guard.
    let pair = (0, 0);
    guard_example1(pair);   // Satisfies `x == y` and `x + y == 0` but prints 'There are twins'.

    // The compiler cannot check the match expression covers all possible conditions anymore,
    // so you MUST use the `_` pattern at the end.
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => panic!("Fell through"), // This should not be possible to reach
    }
}
