Rust 在`no_std` 环境下打印hello world。

在不同系统环境下分别使用`cargo run --release`进行效果测试。

在linux/mac下有一个不同调用方式的features，通过`cargo run --release --features asm`进行测试。

已在win10平台和linux平台上验证，~~因手边没有mac，所以未能测试mac的部分~~(mac部分已测试)。代码均查看自官方文档。

水平有限，有不正确的地方希望大家指正。



更新：

mac部分已经在虚拟机中测试完成。由于cargo目前还不能支持读取feature,参见[该issue](https://github.com/rust-lang/cargo/issues/8170)，所以不能直接使用`cargo build`进行编译。编译命令如下：

```shell
# 动态链接到libc，通过libc进行系统调用。otool打印如下：
# Ynits-iMac:rust-nostd-helloworld-master ynit$ otool -L target/release/rust-nostd-helloworld
# target/release/rust-nostd-helloworld:

cargo rustc --release -- -C link-args="-e __start -nostartfiles"

# 手写汇编代码进行系统调用，直接整个静态链接。otool打印如下：
# Ynits-iMac:rust-nostd-helloworld-master ynit$ otool -L target/release/rust-nostd-helloworld
# target/release/rust-nostd-helloworld:
#	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1225.1.1)

cargo rustc --release --features=asm -- -C link-args="-e __start -static -nostartfiles"
```







本项目为了验证rust在windows/linux/mac平台下`no_std`的情况。起因是看到了[这篇文章](https://rustmagazine.github.io/rust_magazine_2021/chapter_3/no_std_binary.html)。

首先文章中说到：

> 目前无论是C语言还是 Rust, 仅在 linux 系统下能编译 no_std 的可执行文件，用 mac 或 windows 系统的读者要装 linux 虚拟机才能学习 no_std

我认为并不是这样的，我觉得no_std和平台并没有任何关系。我是这样思考的，std实际上是比no_std更多的存在，既然std都能支持，那么no_std更应该没问题才对。

继续往下看文章：

>mac/windows 都能编译 no_std 的 library,但是 mac 运行 no_std 的 binary 会报错`illegal hardware instruction`
>
>RustChinaConf 2020的Rust, RISC-V和智能合约中展示了一个 Rust 最简单的 no_std 可执行文件
>
>我私下问过该topic演讲嘉宾，为什么 PPT 上的 no_std 代码在 mac 上运行会报错`illegal hardware instruction`
>
>嘉宾建议我在 linux 系统下运行，我换 linux 后果然就正常运行了

看到这里，我大概就明白了，作者拿着linux的系统调用在mac上跑，当然不行了，而且也给出了明确的错误提示`illegal hardware instruction`。

继续往下：

> no_std 可执行文件意味着不能依赖操作系统的动态链接库，意味着可执行文件将是纯 statically_linked_executable

> 总结
>
> 当前 Rust 的 no_std 生态仅在 linux 上比较完善，Rust/C/C++ 在 no_std 环境下想要打印`Hello World`还得用汇编指令 syscall 系统调用，需要开发者对汇编语言和操作系统有一定的了解才能在 no_std 环境下开发

这个我认为也是不正确的，no_std和动态链接没有直接的关系，这个库代码中分别给出了直接系统调用（汇编方式。windows并没有提供系统调用的资料和保证，而且内部一直在变化，所以没有）和链接库进行调用的操作。

结论，我感觉作者从一开始的运行错误，导致没能正确的认识no_std，于是后面得出了一系列有偏差的结论。