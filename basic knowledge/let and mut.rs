
fn main() {              // Rust 中默认变量是不可变的
    let mut x = 5; //mut 关键字表示变量可变 //let 关键字用于声明变量
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
}
