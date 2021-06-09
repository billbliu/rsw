# rws
rust项目练习，参考https://gitee.com/dev-tang/rsw

# RustWriter
这是一个用Rust语言编写的静态博客生成工具。追求简单、自由、快乐。

### 安装
Linux：下载rsw文件复制到/usr/local/bin命令下，然后就可使用RustWriter

- 下载源代码
```
git clone https://github.com/billbliu/rsw.git
cd rsw
```
- 编译代码
```
cargo build --release
```
- Linux安装
```
sudo cp target/release/rsw   /usr/local/bin/
```

### 使用

- rsw -h 查看帮助
- rsw -V 显示版本信息
- rsw new project 创建一个静态博客项目
- rsw build 编译src目录下的文件到build

### 案例
[rsw-example](https://github.com/tjz101/rsw-example)