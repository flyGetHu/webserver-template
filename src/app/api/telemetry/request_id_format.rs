use std::fmt;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::{
    format::{FormatFields, Writer},
    FmtContext, FormatEvent,
};
use tracing_subscriber::registry::LookupSpan;

use crate::app::api::telemetry::RequestId;

#[derive(Debug, Clone)]
pub struct RequestIdFormat;

impl<S, N> FormatEvent<S, N> for RequestIdFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // 获取当前时间
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");

        // 写入时间戳
        write!(writer, "{} ", timestamp)?;

        // 写入日志级别
        let level = match *event.metadata().level() {
            Level::TRACE => "TRACE",
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
        };
        write!(writer, "{:>5} ", level)?;

        // 尝试从当前span的作用域中获取根span的request_id
        let request_id = ctx
            .event_scope()
            .and_then(|scope| {
                // 获取根span（即第一个span）
                scope.from_root().next().and_then(|span| {
                    span.extensions()
                        .get::<RequestId>()
                        .map(|req_id| req_id.0.to_string())
                })
            })
            .unwrap_or_else(|| "none".to_string());

        // 写入 request_id
        write!(writer, "[request_id={}] ", request_id)?;

        // 写入目标（模块路径）
        if let Some(module_path) = event.metadata().module_path() {
            write!(writer, "{}: ", module_path)?;
        }

        // 写入日志内容
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
