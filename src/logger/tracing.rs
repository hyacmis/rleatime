#[cfg(test)]
mod test_tracing {
    use tracing::{info, span, Level};
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    #[test]
    fn init() {
        // 只有注册 subscriber 后， 才能在控制台上看到日志输出
        tracing_subscriber::registry().with(fmt::layer()).init();

        // 调用 `log` 包的 `info!`
        log::info!("Hello world");

        let foo = 42;
        // 调用 `tracing` 包的 `info!`
        info!("Hello from tracing {foo}");
    }

    #[test]
    fn span() {
        tracing_subscriber::registry().with(fmt::layer()).init();

        let span = span!(Level::TRACE, "my_span");

        // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
        let _enter = span.enter();
        // 这里开始进入 `my_span` 的上下文
        // 下面执行一些任务，并记录一些信息到 `my_span` 中
        // ...
        info!("hello world");
        info!("hello rust");
    } // 这里 enter 将被 drop，`my_span` 也随之结束
}
