# vicyRust
一个关于rust的仓库，我将在这里分享我自己学习rust的文件如果你感兴趣的话可以打开看看（本人是小白肯定有不懂的，编程大佬还是看看别的吧）祝你天天开心


里面的文件是以学习为目的的，不涉及商业目的



🦀 Rust 安装指南




本项目的开发基于 Rust 编程语言。

以下是不同操作系统下快速安装 Rust 的方法。


🔧 推荐安装方式（所有平台）



1. 使用 rustup 工具链管理器（官方推荐）

```bash
# Linux / macOS / WSL 默认安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows PowerShell 管理员模式
winget install Rustlang.Rustup

# 完成安装后重载环境变量
source $HOME/.cargo/env   # Linux/macOS
# 或重新打开终端               # Windows
```

2. 自定义安装（可选）


在有些情况下   不是y , n，c  而是 1，2，3 他们是对应的。


y就对应1，n就对应2, c就对应3


在一般情况下默认按 1 就可以了

y：按回车确认安装至默认路径并选择稳定版（rustup default toolchain）。


n：停止安装过程


c：进入自定义安装界面，可以选择组件或更改安装路径



如需手动添加环境变量（如未自动生效）

```bash
export PATH="$HOME/.cargo/bin:$PATH"


```
🌐 跨平台安装方式

✅ Linux

```bash
# Ubuntu / Debian 系统
curl https://sh.rustup.rs -sSf | sh
```


✅ macOS

```bash

# Homebrew 也可用
brew install rustup-init
```
✅ Windows

```bash

# 方法一：winget 安装
winget install --id Rustlang.Rustup

# 方法二：浏览器下载 rustup-init.exe 双击运行

https://rust-lang.org/   这个是下载链接,然后直接点击 GET STARTED  然后等待安装即可
```

🧪 验证安装
安装完成后，请确保执行以下命令检查是否安装成功


```bash

rustc --version    # 编译版本号
cargo --version    # 构建工具版本号
```

预期输出示例
（由于rust的更新速率是挺快的，所以你看到的可能不是这个型号但是样子应该是差不多的）


```bash

rustc 1.74.0-nightly (a59d5b6e8 2023-08-27)
cargo 1.74.0-nightly (078b1d74e 2023-08-27)
```


⚙️ IDE 与工具链配置
🖥 VS Code 推荐插件


```

名称:Rust Analyzer  描述:完整代码智能提示和编辑支持 (这个是肯定要装的)

（下面是可选的）
名称:CodeLLDB      描述:LLDB 调试器支持，用于 C/C++/Rust 等语言断点调试
名称:Dependi       描述:代码依赖可视化与分析工具
名称:Error Lens    描述:实时行内错误与警告提示显示
```
安装步骤：


1.在 VS Code 中打开扩展面板（Ctrl+Shift+X）


2.搜索 "Rust Analyzer"


3.点击安装即可


❓ 常见问题 FAQ
```bash
问题: rustup: command not found   解决方案: 检查 .cargo/bin 是否加入 PATH
问题: macOS Gatekeeper 报错       解决方案: 允许设置 → 安全性与隐私
问题: Windows 无响应              解决方案: 以“管理员身份”打开终端并重新运行安装脚本
```

🛠 可选推荐（进阶）


Clippy: 代码静态分析工具

```bash
rustup component add clippy
```

格式化工具: 自动统一代码风格
```bash
rustup component add rustfmt
```

更新工具链
```bash
rustup update stable  (更新稳定的 Rust 编译器和 Cargo 到最新版本)  需要定期执行
rustup self update    (更新 rustup 工具本身（管理器程序）)         建议偶尔执行一次即可
```

📚 参考资料


1.Rust 官方文档     https://doc.rust-lang.net/stable/


2.Rust by Example  https://doc.rust-lang.org/stable/rust-by-example/


3.Cargo Book       https://doc.rust-lang.org/cargo/





