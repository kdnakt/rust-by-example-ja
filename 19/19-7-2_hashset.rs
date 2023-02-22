use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if
    // there was a value already present.
    // 既に存在する値を追加しようとすると
    // `HashSet::insert()`はfalseを返す。
    //assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう。

    b.insert(5);

    // If a collection's element type implements `Debug`,
    // then the collection implements `Debug`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
    // 集合の要素が、`Debug`を実装している型の場合、
    // 集合そのものも`Debug`を実装する。
    // 通常は`[elem1, elem2, ...]`のように要素をプリントする。
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1, 2, 3, 4, 5] in arbitrary order
    // [1, 2, 3, 4, 5]を順不同にプリント
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This should print [1]
    // これは[1]をプリント
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2, 3, 4] in arbitrary order.
    // [2, 3, 4]を順不同にプリント
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // Print [1, 5]
    // [1, 5]をプリント
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
