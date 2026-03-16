//Rust 中的每个值都属于某个特定的数据类型，这告诉 Rust 是什么类型的数据 数据被指定是为了知道如何处理这些数据。
//Rust 是一种静态类型语言，这意味着所有的变量必须有一个确定的类型，在编译时就知道了这个类型是什么。
//Rust 可以通过类型推断来推断出变量的类型，但有时候我们需要显式地指定类型。

fn main() {
    //Rust 中的整数类型有多种，包括有符号整数（i8、i16、i32、i64、i128）和无符号整数（u8、u16、u32、u64、u128）。每种类型都有不同的位数和范围。
    let x: i32 = 42; // 显式指定为 32 位有符号整数
    let y: u64 = 100; // 显式指定为 64 位无符号整数

    //Rust 中的浮点数类型有两种：f32 和 f64。f32 是单精度浮点数，f64 是双精度浮点数。
    let a: f32 = 3.14; // 显式指定为 32 位浮点数
    let b: f64 = 2.71828; // 显式指定为 64 位浮点数

    //Rust 中的布尔类型是 bool，它只有两个值：true 和 false。
    let is_rust_fun: bool = true; // 显式指定为布尔类型

    //Rust 中的字符类型是 char，它表示一个 Unicode 字符，占用 4 个字节。
    let letter: char = 'R'; // 显式指定为字符类型

    println!(
        "x: {}, y: {}, a: {}, b: {}, is_rust_fun: {}, letter: {}",
        x, y, a, b, is_rust_fun, letter
    );
}

//数值运算
//Rust 支持基本的数值运算，包括加法、减法、乘法、除法和取模运算。我们可以使用这些运算符来对数值类型进行操作。

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );
}

//布尔运算
//Rust 支持基本的布尔运算，包括逻辑与（&&）、逻辑或（||）和逻辑非（!）。我们可以使用这些运算符来对布尔类型进行操作。

fn main() {
    let t = true;
    let f = false;

    // logical AND
    let and_result = t && f; // false

    // logical OR
    let or_result = t || f; // true

    // logical NOT
    let not_result = !t; // false

    println!(
        "and_result: {}, or_result: {}, not_result: {}",
        and_result, or_result, not_result
    );
}

//角色类型
//Rust 中的字符类型是 char，它表示一个 Unicode 字符，占用 4 个字节。我们可以使用单引号来定义一个 char 类型的变量。

fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
}

//复合类型
//Rust 中的复合类型包括元组（tuple）和数组（array）。元组可以包含不同类型的值，而数组只能包含相同类型的值。

//元组
//元组是一种将多个值组合成一个复合类型的方式。我们可以使用圆括号来定义一个元组，并且可以包含不同类型的值。

fn main() {
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (x, y, z) = tup; // 解构元组
    println!("The value of y is: {}", y);
}

//数组
//数组是一种将多个值组合成一个复合类型的方式，但数组中的所有值必须具有相同的类型。
//我们可以使用方括号来定义一个数组，并且需要指定数组的长度。

//当你想把数据分配到栈上时，数组很有用，就像 到目前为止我们见过的其他类型，而不是堆积
// 你要确保元素数量始终固定。一个数组 不过它没有矢量类型那么灵活。向量是类似的集合 标准库提供的类型允许在中增长或缩小 体积是因为它的内容存储在堆上。
//如果你不确定是否应该使用 数组或向量，你很可能应该用向量。

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 定义一个包含 5 个 i32 类型元素的数组
    let first = a[0]; // 访问数组的第一个元素
    println!("The first element of the array is: {}", first);
}

//数组元素访问
//我们可以使用索引来访问数组中的元素。数组的索引从 0 开始，因此第一个元素的索引是 0，第二个元素的索引是 1，以此类推。

fn main() {
    let a = [1, 2, 3, 4, 5];
    let second = a[1]; // 访问数组的第二个元素
    println!("The second element of the array is: {}", second);
}

//无效的数组访问
//如果我们尝试访问数组中不存在的索引，Rust 会在编译时抛出一个错误。这是 Rust 的安全特性之一，可以防止我们访问无效的内存地址。

fn main() {
    let a = [1, 2, 3, 4, 5];
    //let invalid_access = a[10];    // 这行代码会导致编译错误，因为索引 10 超出了数组的范围
    println!("This line will not be reached due to the error above.");
}
