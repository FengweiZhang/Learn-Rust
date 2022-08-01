// enum Coin{
//     Pp,
//     Tt,
//     Cc,
// }

// fn convert(coin: Coin) -> u8 {
//     match coin{
//         Coin::Pp=>1,
//         Coin::Tt=>2,
//         Coin::Cc=>3,
//     }
// }

fn plus_one(input: Option<u32>) -> Option<u32> {
    match input {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    // let s = Coin::Pp;
    // println!("{}", convert(s));
    let a = Some(1);
    let b = plus_one(a);
    println!("{:?}", a);
    println!("{:?}", b);

    let v = Some(0u8);

    if Some(0) == v {
        println!("three");
    };
}
