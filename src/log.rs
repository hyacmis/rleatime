/// log 仅仅是日志门面库，它并不具备完整的日志库功能！
/// 因此你无法在控制台中看到任何日志输出

#[cfg(test)]
mod test_log {
    use log::{debug, info, log, trace, warn, Level};

    #[test]
    fn base() {
        trace!("这是一条 trace 日志");
        info!("这是一条 info 日志");
        warn!("这是一条 warning 日志");
        debug!("这是一条 debug 日志");

        log!(Level::Error, "这是一条可指定 level 的日志");
        log!(target: "app_events", Level::Warn, "这是一条 可配置 的日志");
    }
}
