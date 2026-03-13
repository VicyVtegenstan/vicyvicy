
//像不可变变量一样，常数是绑定在名称和 不允许改变，但常数之间有一些差异 以及变量
//常数必须有类型注解
//现在。只要知道你一定要标注字体。mut const let

//常量可以在任何作用域中声明，包括全局作用域，这使得 它们对许多代码部分需要了解的值非常有用。
//最后一个区别是常数可以只设为常数表达式， 而不是只能在运行时计算的值的结果

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "常数 THREE_HOURS_IN_SECONDS 的值是: {}",
        THREE_HOURS_IN_SECONDS
    );
}
