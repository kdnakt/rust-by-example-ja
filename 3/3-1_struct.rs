#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
// ユニット
struct Nil;

// A tuple struct
// タプル
struct Pair(i32, f32);

// A struct with two fields
// 2つのフィールドを持つ（クラシックな）構造体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// 構造体は他の構造体のフィールドになることができる
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) {
    let Rectangle{top_left: Point{x: x1, y: y1}, bottom_right: Point{x: x2, y: y2}} = rectangle;
    println!("Total: {}", ((x2 - x1) * (y2 - y1)).abs());
}

fn square(point: Point, len: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x: point.x, y: point.y + len },
        bottom_right: Point { x: point.x + len, y: point.y }
    }
}


fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    // pointのフィールドにアクセスする。
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // let bottom_right = Point { ..point, x: 5.2 };  NG
    // let bottom_right = Point { point, x: 5.2 };  NG

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // `let`を使用してpointをデストラクトする。
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        // 構造体の定義とインスタンスの作成を同時に行う
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("third point: ({}, {})", _rectangle.top_left.x, _rectangle.top_left.y);

    // Instantiate a unit struct
    // ユニットをインスタンス化
    let _nil = Nil;

    // Instantiate a tuple struct
    // タプルをインスタンス化
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1); 
    println!("pair contains {:} and {:}", pair.0, pair.1);
    println!("pair contains {} and {}", pair.0, pair.1);

    // Destructure a tuple struct
    // タプルをデストラクト
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // 演習1: Rectangle の面積を計算する rect_area 関数を追加してください。ネストしたデストラクトを使ってみましょう。
    rect_area(_rectangle);
    
    // 演習2: Point と f32 を引数とした時に Rectangle を返す square 関数を追加してください。 Rectangle の左下の点が Point になり、f32 が Rectangle の幅と高さになります。
    print!("{:?}", square(Point{x: 1.0, y: 1.0}, 1.0));

}
