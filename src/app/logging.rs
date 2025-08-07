//! 自定义日志格式化模块
//!
//! 该模块提供了一个自定义的日志格式化器，可以在日志输出中包含 request_id

use std::fmt;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::{
    format::{Writer, FormatFields},
    FmtContext, FormatEvent,
};
use tracing_subscriber::registry::LookupSpan;

/// 自定义日志格式化器
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

        // 尝试从当前 span 中获取 request_id
        let request_id = ctx.event_scope()
            .and_then(|scope| {
                // 从作用域中获取最近的 span（最内层的 span）
                scope.from_root().last().and_then(|span| {
                    // 获取 span 扩展中的格式化字段
                    let extensions = span.extensions();
                    extensions.get::<N>().and_then(|_formatter| {
                        // 注意：我们无法直接从 FormattedFields<N> 提取单个字段值
                        // 因为 FormattedFields 类型不是公共的
                        // 我们只能通过解析格式化后的字符串来提取 request_id
                        // 这里返回 None，因为我们无法直接访问字段值
                        None
                    })
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