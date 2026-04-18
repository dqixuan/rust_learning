
fn main() {
    let version4 = IpAddrKind::V4;
    let version6 = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number: Option<i32> = None; // i32 null

    // if let  只考虑一种情况
    let some_u8_value = Some(0u8); 
    if let Some(3) = some_u8_value { 
        println!("three");
    }

    let coin = CoinV2::Penny;
    let mut count = 0;
    if let coin = Coin::Quarter {

    } else {
        count += 1;
    }

}

// key word
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// we can put anything inside a enum
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum can have associated methods as structs.
impl Message {
    fn call(&self) {

    }
}

enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

// match arms,  match pattern and code block, if code block has multiple lines, we should use curely brackets
fn count_int_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
            println!("hello match");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 15,
        Coin::Quarter => 25,
    }
}

enum CoinV2 {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derived[debug]]
enum UsState {
    Alabama,
}

fn values_int_cents_v2(coin: CoinV2) -> u8 {
    match coin {
        CoinV2::Penny => 1,
        CoinV2::Nickle => 5,
        CoinV2::Dime => 10,
        CoinV2::Quarter(state) => {
            println!("statue quarter is {state:?}!")
            25
        }
        _ => 0,  // _ 表示通配符，表示不属于上述的所有情况
    }
}

fn plus_one(param: Option<i32>) -> Option<i32> {
    match param { // exhaustive  枚举所有的情况
        None => None,
        Some(i) => Some(i+1),
    }
}

// a function with one parameter with one lifetime parameter
fn foo<'a> (x: &'a i32) {}
fn foo1<'a, 'b>(x: &'a i32, y: &'a i32) {}

fn foo2<'a>(x: &'a i32) -> &'a i32 {}





