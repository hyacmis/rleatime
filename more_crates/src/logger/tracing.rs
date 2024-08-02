#![allow(dead_code)]

use chrono::Local;
use tracing::{debug, error, info, subscriber, trace, warn, Level};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    fmt::{self, format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}
fn simple_tracing() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    // 这是最简单的注册方式
    tracing_subscriber::registry().with(fmt::layer()).init();

    // 调用 `tracing` 包的 `info!`
    info!("init tracing"); // 2024-08-01T03:58:39.843010Z  INFO more_crates::logger::tracing: init tracing
    error!("init tracing"); // 2024-08-01T03:58:39.843107Z ERROR more_crates::logger::tracing: init tracing
    warn!("init tracing"); // 2024-08-01T03:58:39.843124Z  WARN more_crates::logger::tracing: init tracing
    debug!("init tracing"); // 2024-08-01T03:58:39.843135Z DEBUG more_crates::logger::tracing: init tracing
    trace!("init tracing"); // 2024-08-01T03:58:39.843145Z TRACE more_crates::logger::tracing: init tracing
}

fn complex_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE) // 最大日志级别
        .with_ansi(true) // 显示ansi的颜色输出
        .with_timer(LocalTimer) // 日期格式
        .with_file(false) // 显示源代码文件路径
        .with_line_number(true) // 显示源代码行号
        .with_thread_ids(true) // 显示记录事件的线程ID
        .with_target(true) // 显示事件的目标（模块路径）
        .compact() // 使用更紧凑、缩写的日志格式
        // .pretty() // 漂亮的多行日志,用于本地开发和调试
        .finish(); // 建立订阅服务器

    // 设置为全局SubScriber
    subscriber::set_global_default(subscriber).unwrap();

    info!("complex tracing");
    error!("complex tracing");
    warn!("complex tracing");
    debug!("complex tracing");
    trace!("complex tracing");

    // compact:
    // 2024-08-01 14:09:17  INFO ThreadId(02) more_crates::logger::tracing:51: complex tracing
    // 2024-08-01 14:09:17 ERROR ThreadId(02) more_crates::logger::tracing:52: complex tracing
    // 2024-08-01 14:09:17  WARN ThreadId(02) more_crates::logger::tracing:53: complex tracing
    // 2024-08-01 14:09:17 DEBUG ThreadId(02) more_crates::logger::tracing:54: complex tracing
    // 2024-08-01 14:09:17 TRACE ThreadId(02) more_crates::logger::tracing:55: complex tracing

    // pretty:
    // 2024-08-01 14:10:35  INFO more_crates::logger::tracing: complex tracing
    //   at more_crates/src/logger/tracing.rs:51 on ThreadId(2)
    //
    // 2024-08-01 14:10:35 ERROR more_crates::logger::tracing: complex tracing
    //   at more_crates/src/logger/tracing.rs:52 on ThreadId(2)
    //
    // 2024-08-01 14:10:35  WARN more_crates::logger::tracing: complex tracing
    //   at more_crates/src/logger/tracing.rs:53 on ThreadId(2)
    //
    // 2024-08-01 14:10:35 DEBUG more_crates::logger::tracing: complex tracing
    //   at more_crates/src/logger/tracing.rs:54 on ThreadId(2)
    //
    // 2024-08-01 14:10:35 TRACE more_crates::logger::tracing: complex tracing
    //   at more_crates/src/logger/tracing.rs:55 on ThreadId(2)
}

fn json_tracing() {
    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/latest/tracing_appender/index.html
    let file_appender = rolling::daily(".", "log");
    let (non_blocking, _guard) = non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE) // 最大日志级别
        .with_writer(non_blocking)
        .with_ansi(true) // 显示ansi的颜色输出
        .with_timer(LocalTimer) // 日期格式
        .with_file(false) // 显示源代码文件路径
        .with_line_number(true) // 显示源代码行号
        .with_thread_ids(true) // 显示记录事件的线程ID
        .with_target(true) // 显示事件的目标（模块路径）
        .json() // 使用 json 格式
        .flatten_event(true) // 展平事件元数据
        .finish();
    subscriber::set_global_default(subscriber).unwrap();

    info!("json tracing");
    error!("json tracing");
    warn!("json tracing");
    debug!("json tracing");
    trace!("json tracing");

    // json:
    // {"timestamp":"2024-08-01 14:55:52","level":"INFO","message":"json tracing","target":"more_crates::logger::tracing","line_number":98,"threadId":"ThreadId(2)"}
    // {"timestamp":"2024-08-01 14:55:52","level":"ERROR","message":"json tracing","target":"more_crates::logger::tracing","line_number":99,"threadId":"ThreadId(2)"}
    // {"timestamp":"2024-08-01 14:55:52","level":"WARN","message":"json tracing","target":"more_crates::logger::tracing","line_number":100,"threadId":"ThreadId(2)"}
    // {"timestamp":"2024-08-01 14:55:52","level":"DEBUG","message":"json tracing","target":"more_crates::logger::tracing","line_number":101,"threadId":"ThreadId(2)"}
    // {"timestamp":"2024-08-01 14:55:52","level":"TRACE","message":"json tracing","target":"more_crates::logger::tracing","line_number":102,"threadId":"ThreadId(2)"}
}

fn console_and_files_tracing() {
    let file_appender = rolling::daily(".", "log");
    let (non_blocking, _guard) = non_blocking(file_appender);

    let console_builder = fmt::layer()
        // .with_max_level(Level::TRACE)
        .with_ansi(true) // 显示ansi的颜色输出
        .with_timer(LocalTimer) // 日期格式
        .with_file(false) // 显示源代码文件路径
        .with_line_number(true) // 显示源代码行号
        .with_thread_ids(true) // 显示记录事件的线程ID
        .with_target(true) // 显示事件的目标（模块路径）
        .compact();

    let file_builder = fmt::layer()
        // .with_max_level(Level::TRACE)
        .with_ansi(true) // 显示ansi的颜色输出
        .with_timer(LocalTimer) // 日期格式
        .with_file(false) // 显示源代码文件路径
        .with_line_number(true) // 显示源代码行号
        .with_thread_ids(true) // 显示记录事件的线程ID
        .with_target(true) // 显示事件的目标（模块路径）
        .with_writer(non_blocking)
        .json()
        .flatten_event(true);

    // 无法为其指定最大日志等级
    let builder = tracing_subscriber::registry()
        .with(console_builder)
        .with(file_builder)
        .with(EnvFilter::new("info"));

    subscriber::set_global_default(builder).unwrap();

    info!("console and files tracing");
    error!("console and files tracing");
    warn!("console and files tracing");
    debug!("console and files tracing");
    trace!("console and files tracing");
}

#[cfg(test)]
mod test_register {
    use super::*;
    #[test]
    #[ignore]
    fn test_simple_tracing() {
        simple_tracing()
    }

    #[test]
    #[ignore]
    fn test_complex_tracing() {
        complex_tracing()
    }

    #[test]
    #[ignore]
    fn test_json_tracing() {
        json_tracing()
    }

    #[test]
    #[ignore]
    fn test_console_and_files_tracing() {
        console_and_files_tracing()
    }
}

#[cfg(test)]
mod test_tracing {
    use tracing::{error, info, span, warn, Level};
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_span() {
        let span = span!(Level::TRACE, "my_span");

        // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
        let _enter = span.enter();
        // 这里开始进入 `my_span` 的上下文
        // 下面执行一些任务，并记录一些信息到 `my_span` 中
        // ...
        info!("hello world");
        assert!(logs_contain("world"));
        logs_assert(|lines: &[&str]| {
            let str = lines.last().unwrap();
            assert!(str.contains("my_span"));
            assert!(str.contains("hello world"));
            assert!(str.contains("INFO"));
            Ok(())
        });

        error!("hello rust");
        assert!(logs_contain("rust"));
        logs_assert(|lines: &[&str]| {
            let str = lines.last().unwrap();
            assert!(str.contains("my_span"));
            assert!(str.contains("rust"));
            assert!(str.contains("ERROR"));
            Ok(())
        });
    } // 这里 enter 将被 drop，`my_span` 也随之结束

    #[traced_test]
    #[tokio::test]
    async fn test_log() {
        // 主线程的日志
        info!("info level");

        // 子线程中的日志
        tokio::spawn(async {
            warn!("warn level");
        })
        .await
        .unwrap();

        // Ensure that certain strings are or aren't logged
        assert!(logs_contain("info level"));
        assert!(logs_contain("warn level"));
        assert!(!logs_contain("error level"));
    }
}
