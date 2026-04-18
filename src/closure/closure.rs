#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    FnOnce: 只能被调用过一次
    FnMut:  修改获取的变量
    Fn: 
*/

fn main() {
    // FnMut
    let mut list = [
        Rectangle{width: 10, height: 1},
        Rectangle{width: 3, height: 5},
        Rectangle{width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // FnOnce
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    let mut count = 0;

    list.sort_by_key(|r| {
        // sort_operations.push(value); 无法多次使用，只能使用一次
        count += 1;
        r.height
    });

    println!("{:#?}", list);
    println!("count is {}", count);
    // println!("{:#?}", list);
}