use configs::CFG;
use silent::prelude::logger::fmt::format::{Compact, Format};
use silent::prelude::logger::fmt::time::ChronoLocal;
use silent::prelude::logger::layer::SubscriberExt;
use silent::prelude::logger::{fmt, EnvFilter, Registry};
use silent::prelude::Level;
use tracing_appender::non_blocking::WorkerGuard;

pub fn init_logs() -> Vec<WorkerGuard> {
    let mut guards = Vec::new();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", &CFG.log.log_level);
    }
    // 系统变量设置
    let log_env = get_log_level();
    //  日志设置
    let format = get_log_format();
    // 文件输出
    let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    guards.push(guard);
    // 标准控制台输出
    let (std_non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(guard);
    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(log_env.into()))
        .with(
            fmt::Layer::default()
                .with_writer(std_non_blocking)
                .event_format(format.clone())
                .pretty(),
        )
        .with(
            fmt::Layer::default()
                .with_writer(non_blocking)
                .event_format(format),
        );
    tracing::subscriber::set_global_default(logger).unwrap();
    guards
}

pub fn get_log_level() -> Level {
    match CFG.log.log_level.as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::INFO,
    }
}

pub fn get_log_format() -> Format<Compact, ChronoLocal> {
    fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(ChronoLocal::rfc_3339())
        .compact()
}
