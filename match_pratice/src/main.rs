// // 填空
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }

// fn main() {
//     let boolean = true;

//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 0,
//     };

//     assert_eq!(binary, 1);
// }

// // 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0),
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// }

// fn show_message(msg: Message) {
//     match msg {
//         Message::Move { x: a, y: b } => {
//             // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         }
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         __ => println!("no data in these variants"),
//     }
// }

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(matches!(ab,'A'..='Z' | 'a'..='z' | '0'..='9'));
//     }
// }

// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if matches!(e,MyEnum::Foo){ // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);
// }

// fn main() {
//     let o = Some(7);

//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     // match o {
//     //     Some(i) => {
//     //         println!("This is a really long string and `{:?}`", i);
//     //     }
//     //     _ => {}
//     // };
//     if let Some(o) = o{
//         println!("This is a really long string and `{:?}`", o);
//     }
// }

// // 填空
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
//         println!("foobar 持有的值是: {}", i);
//     }
// }

// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32),
// }

// fn main() {
//     let a = Foo::Qux(10);

//     // 移除以下代码，使用 `match` 代替
//     // if let Foo::Bar = a {
//     //     println!("match foo::bar")
//     // } else if let Foo::Baz = a {
//     //     println!("match foo::baz")
//     // } else {
//     //     println!("match others")
//     // }
//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match others"),
//     }
// }


// 就地修复错误
fn main() {
    let age = Some(30);
    if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
    //    assert_eq!(age, Some(30));
        assert_eq!(age,30);
    } // 新的 `age` 变量在这里超出作用域
    
    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
 }
