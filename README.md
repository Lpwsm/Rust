# Rust

安装

```
curl https://sh.rustup.rs -sSf | sh
```

将Cargo加入环境变量

```
export PATH="$HOME/.cargo/bin:$PATH"
source $HOME/.cargo/env
```

查看安装版本

```
rustc --version
```

```
# 安装工具链
rustup install nightly-x86_64-pc-windows-gnu

# 查看当前安装的工具链
rustup show

# 更新当前的工具链
rustup update

# 卸载rust
rustup self uninstall
```

修改Rust Crates 源

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'rustcc'

# rustcc 1号源
[source.rustcc]
registry="git://crates.rustcc.com/crates.io-index"

# rustcc 2号源
[source.rustcc2]
registry="git://crates.rustcc.cn/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
```



cargo new 报如下错误

```
could not determine the current user, please set  $USER

# 解决
export USER=root
```

![set_user](.\Imgs\set_user.PNG)

cargo run 报错

```
# 安装gcc
sudo apt install gcc
```

![](.\Imgs\linker_cc.PNG)