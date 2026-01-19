
fn main() {
   let condition: bool = true;

   if condition {
    println!("The value of condition is {}", condition);
   }

   let value = if condition {5} else {6};

   println!("the value is {}", value);


    // loop 
    // Loop labels must begin with a single quote.
    let mut count1 = 0;
    'loop_label: loop {

        let mut count2 = 10;
        println!("current count1 is {}", count1);
        loop {
            println!("current count2 is {}", count2);
            if count2 == 9 {
                break;
            }
            if count1 == 2 {
                break 'loop_label;
            }
            count2 -= 1;
        }
        count1 += 1;
    }


    // while 
    let mut count = 0;
    while count < 5 {
        println!("current count value is {}", count);
        count += 1;
    }

    // for  
    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("the element is {}", element);
    }

    for number in (1..4).rev() {
        println!("reverse order the number is {}", number);
    }

    for number in 10..15 {
        println!("the number is {}", number)
    }


}
// comment  starts with two slashes and 
// control flow: if、else、else if、 loop（break、 label）、 while、 for