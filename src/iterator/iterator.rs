fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // iterator 是一次性的， for in 遍历后，v1_iter已经失效了
    for val in v1_iter {
        println!("Got: {val}")
    }

    let v2_iter = v1.iter();
    let total: i32 = v2_iter.sum();
    println!("total is {total}");


    let v3_iter: Vec<i32>= v1.iter().map(|x| x + 1).collect();

    for val in v3_iter {
        println!("Got: {val}")
    }

    let v4: Vec<&i32> = v1.iter().filter(|x| *x % 3 == 0).collect();
    println!("======");
    for val in v4 {
        println!("Got: {val}")
    }

}

/*
    iterator trait
    pub trait Iterator {
        type Item;
        
        fn next(&mut selft) -> Option<Self::Item>;
        //
        // rust 默认实现
    }
*/

#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 15,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(in_my_size, 
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                }
            ]
        );
    }
}