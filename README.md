编程语言练习

## TODO

[] 从 xmake 切换到 cmake

xmake构建

## C++

```bash
cd cpp
```

编译运行

```
xmake build sptr && xmake run sptr
```

编译

```bash
xmake
```

运行: 不重新编译，运行上次的binary

```bash
xmake run main
```

生成 compile_commands

```bash
xmake project -k compile_commands
```

## rust

```bash
cd rust
```

```
cargo run --bin hello
```

## golang

```bash
cd golang
```

```
go run hello.go
```