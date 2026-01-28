
fn main() {
    println!("hello function");
    another_function();

    function_with_parameter(10);
}

// The file name can not be same as the folder name.
// rust代码中的函数、变量名使用下划线命名法（snake case），所有字母均小写，并使用下滑先分割单词。
// 函数签名，必须声明每个参数类型，多个参数，使用逗号分割。

fn another_function() {
    println!("another function!");
}

fn function_with_parameter(x: i32) {
    println!("print result with parameter {}", x);
}

// 区分 statement、expression
// statement: ends with semicolon,  has no returned value.
// expression: must return a value