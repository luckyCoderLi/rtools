# Rust工具集 (rtools)

这是一个用于学习Rust编程语言的工具集项目，包含了多个实用的命令行工具，帮助您掌握Rust的核心概念。

## 项目结构

```
rtools/
├── src/
│   ├── lib.rs           # 库代码，包含所有工具的核心逻辑
│   ├── main.rs          # 二进制入口，只负责命令行解析
│   ├── fileinfo.rs      # 文件信息查看模块
│   ├── textstats.rs     # 文本统计模块
│   └── dirscan.rs       # 目录扫描模块
├── tests/               # 集成测试
│   ├── fileinfo_tests.rs
│   ├── textstats_tests.rs
│   └── dirscan_tests.rs
├── examples/            # 使用示例
│   └── basic_usage.rs
├── Cargo.toml           # 项目配置
└── README.md           # 项目说明
```

## 可用工具

### 1. 文件信息查看器 (fileinfo)
显示指定文件的详细信息，包括大小、类型、修改时间等。

**使用方法:**
```bash
cargo run -- fileinfo <文件路径>
```

**学习要点:**
- 文件系统操作 (`std::fs`)
- 错误处理 (`Result<T, E>`)
- 路径处理 (`std::path::Path`)
- 时间处理 (`std::time`)

### 2. 文本统计工具 (textstats)
分析文本文件的统计信息，包括字符数、单词数、词频等。

**使用方法:**
```bash
cargo run -- textstats <文件路径>
```

**学习要点:**
- 字符串处理 (`String`, `&str`)
- 集合类型 (`HashMap`, `Vec`)
- 迭代器 (`Iterator`)
- 结构体 (`struct`) 和实现 (`impl`)
- 模式匹配 (`match`)

### 3. 目录扫描器 (dirscan)
递归扫描目录，统计文件信息、大小分布、文件类型等。

**使用方法:**
```bash
cargo run -- dirscan <目录路径> [最大深度]
```

**学习要点:**
- 递归函数
- 文件系统遍历
- 复杂数据结构
- 所有权和借用 (`ownership & borrowing`)
- 生命周期 (`lifetimes`)

### 4. 文件搜索工具 (filesearch)
在目录中搜索文件，支持按名称、扩展名、大小等条件过滤。

**使用方法:**
```bash
cargo run -- filesearch <目录路径> [选项]
# 选项:
#   --name <模式>    按文件名模式搜索
#   --ext <扩展名>   按文件扩展名搜索
#   --size <范围>    按文件大小搜索 (如: 1000-5000)
#   --depth <深度>   限制搜索深度
```

**学习要点:**
- 构建器模式 (`Builder Pattern`)
- 命令行参数解析
- 递归搜索算法
- 条件匹配和过滤

### 5. 日志分析工具 (loganalyzer)
分析日志文件，统计日志级别、时间分布、错误模式等。

**使用方法:**
```bash
cargo run -- loganalyzer <日志文件>
```

**学习要点:**
- 时间处理 (`chrono`)
- 文本解析和模式匹配
- 枚举类型 (`enum`)
- 复杂数据分析

### 6. 配置管理器 (config)
管理各种格式的配置文件，支持JSON、TOML、INI等格式。

**使用方法:**
```bash
cargo run -- config <配置文件>           # 显示配置内容
cargo run -- config <配置文件> get <键>  # 获取配置值
cargo run -- config <配置文件> set <键> <值>  # 设置配置值
```

**学习要点:**
- 序列化/反序列化 (`serde`)
- 多格式文件处理
- 嵌套数据结构
- 类型转换和验证

### 7. HTTP客户端工具 (httpclient)
发送HTTP请求，支持GET、POST等方法，查看响应状态、头信息等。

**使用方法:**
```bash
cargo run -- httpclient <URL> [选项]
# 选项:
#   --method <方法>  指定HTTP方法 (GET, POST, PUT, DELETE, HEAD, OPTIONS)
#   --body <内容>    指定请求体
#   --timeout <秒数> 指定超时时间
```

**学习要点:**
- 异步编程 (`async/await`)
- HTTP协议和网络请求
- 错误处理和超时
- 请求/响应处理

### 8. 网络连接测试工具 (network)
测试网络连接，支持ping、端口扫描、DNS查询等功能。

**使用方法:**
```bash
cargo run -- network <主机> [选项]
# 选项:
#   --ping                   执行ping测试
#   --port <端口>            指定测试端口 (默认80)
#   --scan <起始端口> <结束端口> 扫描端口范围
#   --dns                    执行DNS查询
#   --count <次数>           指定ping次数 (默认4)
```

**学习要点:**
- TCP/UDP网络编程
- 并发编程 (`tokio`)
- DNS解析和网络诊断
- 异步I/O操作

## 运行示例

```bash
# 查看帮助信息
cargo run -- help

# 基础工具
cargo run -- fileinfo src/main.rs
cargo run -- textstats src/main.rs
cargo run -- dirscan src/ 1

# 高级工具
cargo run -- filesearch src/ --ext rs
cargo run -- loganalyzer example.log
cargo run -- config example_config.json

# 网络工具
cargo run -- httpclient https://api.github.com
cargo run -- httpclient https://httpbin.org/post --method POST --body '{"test": "data"}'
cargo run -- network google.com --ping
cargo run -- network localhost --scan 80 100
cargo run -- network google.com --dns

# 配置管理
cargo run -- config example_config.json get app.name
cargo run -- config example_config.json set server.port 8080

# 运行示例程序
cargo run --example basic_usage
cargo run --example advanced_usage
cargo run --example network_usage

# 运行测试
cargo test
```

## Rust核心概念练习

通过这个项目，您可以练习以下Rust核心概念：

### 基础语法
- 变量声明和类型推断
- 函数定义和调用
- 控制流 (`if`, `match`)
- 错误处理 (`Result`, `?` 操作符)

### 内存管理
- 所有权系统
- 借用规则
- 生命周期
- 智能指针

### 数据结构
- 基本类型 (`String`, `Vec`, `HashMap`)
- 结构体定义和实现
- 枚举类型
- 特征 (`trait`)

### 高级特性
- 迭代器和闭包
- 模块系统
- 错误处理模式
- 特征 (`trait`) 和泛型
- 测试编写
- 文档注释

## 扩展建议

您可以尝试以下扩展来进一步提升Rust技能：

1. **添加新工具:**
   - 文件搜索工具
   - 日志分析器
   - 配置管理器
   - 网络工具

2. **改进现有工具:**
   - 添加更多命令行选项
   - 支持正则表达式
   - 添加进度条显示
   - 实现并行处理

3. **学习新概念:**
   - 异步编程 (`async/await`)
   - 宏编程 (`macro_rules!`)
   - 性能优化
   - 发布到 crates.io

## 学习资源

- [Rust官方文档](https://doc.rust-lang.org/)
- [Rust程序设计语言](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust标准库文档](https://doc.rust-lang.org/std/)

## 贡献

欢迎提交Issue和Pull Request来改进这个项目！

## 许可证

MIT License 