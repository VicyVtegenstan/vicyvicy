
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏");
    println!("请输入一个数字，范围是1到100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("随机数已生成，随机数是{secret_number}");  // 这行代码是用来测试的，可以观察你是否正确的生成了随机数，在正式发布的时候可以注释掉或者删除掉这行代码。
    loop {
        println!("请输入你的猜测数字。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // 这段代码是用来处理用户输入的，如果用户输入的不是一个有效的数字，那么程序会继续循环，直到用户输入一个有效的数字为止。

        println!("你猜测的数字是: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了");
                break; // 这行代码是用来结束循环的，当用户猜对了数字，程序会输出“你赢了”，然后跳出循环，结束程序。
            }
        }
    }
}