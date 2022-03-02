fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    // 1000以下の奇数を2乗した値の合計を求める。
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    // 宣言型プログラミングによるアプローチ
    // 値を蓄積する変数を宣言
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    // 0から無限までイテレートする
    for n in 0.. {
        // Square the number
        // 値を2乗
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            // 上限に達した場合、ループを終了
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            // 奇数ならば値を値を足しあわせていく。
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    // 関数型プログラミングによるアプローチ
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
             .filter(|&n_squared| is_odd(n_squared)      // That are odd
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
                                                         // 全自然数を2乗し
                                                         // そのうち上限より小さい値で
                                                         // かつ奇数のものを
             .fold(0, |acc, n_squared| acc + n_squared); // Sum them
                                                         // 足し合わせる。
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
