use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
// ジェネリックなタプル構造体。2つ目のパラメータは幽霊型
#[derive(PartialEq,Debug)] // Allow equality test for this type.
                     // 比較演算子(`==`)での比較を可能にする。
struct PhantomTuple<A, B>(A,PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`.
// 同様に構造体を定義
#[derive(PartialEq,Debug)] // Allow equality test for this type.
                     // 比較演算子での比較を可能にする。
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }


#[derive(Debug)]
enum Event<A> {
    KeyPress(PhantomData<A>),
}

// Note: Storage is allocated for generic type `A`, but not for `B`.
//       Therefore, `B` cannot be used in computations.
// 注意点:  ジェネリック型Aに対してはメモリが割り当てられているが、
//          Bには割り当てられていないため、計算に使うことはできない。

fn main() {
    // Here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`.
    // <char, f32>と型宣言されたPhantomTupleを作成
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    println!("{:?}", _tuple1); // PhantomTuple('Q', PhantomData)
    // PhantomTuple type specified as `<char, f64>`.
    // <chr, f64>のPhantomTuple。 PhantomDataがいかなる浮動小数点でもないことに注目
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _tuple3: PhantomTuple<f32, f32> = PhantomTuple(1.0, PhantomData);

    // Type specified as `<char, f32>`.
    // <char, f32>の型が与えられた構造体を作成
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    println!("{:?}", _struct1); // PhantomStruct { first: 'Q', phantom: PhantomData }
    // Type specified as `<char, f64>`.
    // 同様に<char, f64>の構造体
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct3: PhantomStruct<char, f64> = PhantomStruct {
        first: 'R',
        phantom: PhantomData,
    };

    // Compile-time Error! Type mismatch so these cannot be compared:
    // コンパイルエラー！型が違うので`==`で比較することができない！
    // println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);

    // println!("_tuple1 == _tuple3 yields: {}",
    //          _tuple1 == _tuple3);

    // Compile-time Error! Type mismatch so these cannot be compared:
    // コンパイルエラー! 型が違うので比較することができない!
    // println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);

    println!("_struct3 == _struct2 yields: {}",
             _struct3 == _struct2); // false


    let event: Event<f64> = Event::KeyPress(PhantomData);
    println!("{:?}", event);
}