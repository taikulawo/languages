探索底层计算机原理

xmake构建

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