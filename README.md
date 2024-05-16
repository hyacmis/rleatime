# runtime

学习 `rust` 语法的私有项目
使用 `RustRover` IDE

# 已完成:

## 基础语法

- [x] `Vec`
- [x] `HashMap`
- [x] `struct` 结构体与impl
- [x] `Enum` 枚举
- [x] Type to Type 类型转换
- [x] iter 各种遍历操作

## 进阶语法

- [x] `static` 全局静态变量
- [x] err 错误
- [x] `T` 泛型
- [x] `trait` 特征
- [x] 高级语法
    - [x] `thread` 多线程
    - [x] thread of channel 线程通道
    - [x] thread of lock 线程锁
    - [x] thread of atomic 原子操作,内存顺序
    - [ ] thread Send 和 Sync 线程安全
    - [x] `Box` 智能指针

## 内部库

- [x] `file` 文件读写
- [x] `log` 日志库规范
- [x] `time` 时间库，提供基础的时间获取方法

## 第三方库

- [x] [`serde`](https://github.com/serde-rs/serde) 序列化
    - [x] to json
    - [x] to enum
    - [x] to struct
- [x] [`tracing`](https://github.com/tokio-rs/tracing) 日志框架，支持OpenTelemetry格式
- [x] [`chrono`](https://github.com/chronotope/chrono) 日期库
    - [x] 实现计算周/月的开始日期(泛型)
- [ ] [`sea-orm`](https://github.com/SeaQL/sea-orm) ORM

## 配置管理

- [x] [`dotenvy`](https://github.com/allan2/dotenvy)  `dotenv` 的分叉 此库从 .env 文件加载环境变量
- [x] [`config`](https://github.com/mehcode/config-rs/tree/master)  用于 Rust 应用程序的分层配置系统

## demo

- [x] snake [贪吃蛇项目](https://github.com/coufxr/snake.git)
- [x] `slint` [betape](https://github.com/coufxr/betape.git)
