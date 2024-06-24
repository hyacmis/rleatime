# runtime

学习 `rust` 语法的项目
使用 `RustRover` IDE

## 语法

- [x] `Vec` 动态数组
- [x] `HashMap` 集合/字典
- [x] `struct` 结构体与 `impl`
- [x] `Enum` 枚举
- [x] Type to Type 类型转换
- [x] iter 各种遍历操作
- [x] `static` 全局静态变量
- [x] `Eeeor` 错误
- [x] `T` 泛型
- [x] `trait` 特征
- [x] 智能指针
    - [x] `Box`
    - [X] `Rc` 和 `Arc` 引用计数
    - [x] `Cell` 和 `RefCell`
- [ ] `thread` 多线程
    - [x] `channel` 线程通道
    - [x] `lock` 线程锁
    - [x] `atomic` 原子操作,内存顺序
    - [ ] `Send` 和 `Sync` 线程安全

## 内部库

- [x] `fs` 文件读写
    - [x] [`csv`](https://github.com/BurntSushi/rust-csv) （第三方）csv处理
- [x] `log` 日志库规范
- [x] `time` 时间库，提供基础的时间获取方法

## 第三方库

- [x] [`serde`](https://github.com/serde-rs/serde) 序列化
    - [x] to json
    - [x] to enum
    - [x] to struct

### 日志记录

- [x] [`tracing`](https://github.com/tokio-rs/tracing) 日志框架，支持OpenTelemetry格式

### 日期计算

- [x] [`chrono`](https://github.com/chronotope/chrono) 日期库
    - [x] 实现计算周/月的开始日期(泛型)

### ORM

- [ ] [`sqlx`](https://github.com/launchbadge/sqlx) 数据库访问库，非ORM
- [ ] [`sea-orm`](https://github.com/SeaQL/sea-orm) 对 `sqlx` 的上层封装

### 错误处理

- [x] [`thiserror`](https://github.com/dtolnay/thiserror) lib使用，自定义错误类型，简化自定义错误类型的定义
- [x] [`anyhow`](https://github.com/dtolnay/anyhow)   bin使用，错误的创建,组合,传递和转换．与 `thiserror` 相辅相成

### 配置管理

- [x] [`dotenvy`](https://github.com/allan2/dotenvy)  `dotenv` 的分叉 此库从 .env 文件加载环境变量
- [x] [`config`](https://github.com/mehcode/config-rs/tree/master)  用于 Rust 应用程序的分层配置系统

### 命令行

- [ ] [`clap`](https://github.com/clap-rs/clap)  命令行参数解析器

## demo

- [ ] snake
    - [x] [demo](snake) 使用 `piston_window` 绘制
    - [ ] [demo2](https://github.com/coufxr/snake.git) 使用 `slint` 绘制
- [ ] `slint`
- [x] `axum` [web-demo](https://github.com/coufxr/web-demo)
- [x] 多线程下载 [download_cli](download_cli)
- [ ] `clap` 实现一个demo
