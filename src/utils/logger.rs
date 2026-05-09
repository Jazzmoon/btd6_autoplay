use std::sync::atomic::Ordering;
use std::time::Instant;

use lazy_static::lazy_static;

use crate::utils::global::LOG_LEVEL;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Emerg = 0,
    Alert = 1,
    Crit = 2,
    Error = 3,
    Warn = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
    /// Lower than `Debug`. Enables image file output in addition to text logs.
    Trace = 8,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_lowercase().as_str() {
            "emerg" | "emergency" => Ok(Self::Emerg),
            "alert" => Ok(Self::Alert),
            "crit" | "critical" => Ok(Self::Crit),
            "error" | "err" => Ok(Self::Error),
            "warn" | "warning" => Ok(Self::Warn),
            "notice" => Ok(Self::Notice),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err(format!("Unknown log level: '{}'", input)),
        }
    }
}

impl LogLevel {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Emerg),
            1 => Some(Self::Alert),
            2 => Some(Self::Crit),
            3 => Some(Self::Error),
            4 => Some(Self::Warn),
            5 => Some(Self::Notice),
            6 => Some(Self::Info),
            7 => Some(Self::Debug),
            8 => Some(Self::Trace),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Emerg => "EMERG",
            Self::Alert => "ALERT",
            Self::Crit => "CRIT",
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Notice => "NOTICE",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }

    fn color_code(&self) -> &'static str {
        match self {
            Self::Emerg => "\x1b[1;37;41m",
            Self::Alert => "\x1b[1;31m",
            Self::Crit => "\x1b[35m",
            Self::Error => "\x1b[31m",
            Self::Warn => "\x1b[33m",
            Self::Notice => "\x1b[36m",
            Self::Info => "\x1b[32m",
            Self::Debug => "\x1b[90m",
            Self::Trace => "\x1b[2;36m",
        }
    }
}

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

pub struct Logger;

impl Logger {
    pub fn set_level(level: LogLevel) {
        LOG_LEVEL.store(level as u8, Ordering::SeqCst);
    }

    pub fn level() -> LogLevel {
        let raw = LOG_LEVEL.load(Ordering::SeqCst);
        LogLevel::from_u8(raw).unwrap_or(LogLevel::Warn)
    }

    pub fn is_enabled(level: LogLevel) -> bool {
        level as u8 <= LOG_LEVEL.load(Ordering::SeqCst)
    }

    pub fn debug(message: impl AsRef<str>) {
        Self::log(LogLevel::Debug, message);
    }

    pub fn trace(message: impl AsRef<str>) {
        Self::log(LogLevel::Trace, message);
    }

    pub fn info(message: impl AsRef<str>) {
        Self::log(LogLevel::Info, message);
    }

    pub fn notice(message: impl AsRef<str>) {
        Self::log(LogLevel::Notice, message);
    }

    pub fn warn(message: impl AsRef<str>) {
        Self::log(LogLevel::Warn, message);
    }

    pub fn error(message: impl AsRef<str>) {
        Self::log(LogLevel::Error, message);
    }

    pub fn crit(message: impl AsRef<str>) {
        Self::log(LogLevel::Crit, message);
    }

    pub fn alert(message: impl AsRef<str>) {
        Self::log(LogLevel::Alert, message);
    }

    pub fn emerg(message: impl AsRef<str>) {
        Self::log(LogLevel::Emerg, message);
    }

    /// Log a message unconditionally, bypassing the configured log level.
    /// Use this only for output that must always be visible, such as
    /// interactive utility messages (e.g. the location finder).
    pub fn force(level: LogLevel, message: impl AsRef<str>) {
        Self::log_inner(level, message);
    }

    fn log(level: LogLevel, message: impl AsRef<str>) {
        if !Self::is_enabled(level) {
            return;
        }
        Self::log_inner(level, message);
    }

    /// Internal implementation that formats and prints a log line without
    /// checking whether the given level is currently enabled.
    fn log_inner(level: LogLevel, message: impl AsRef<str>) {
        let elapsed = START_TIME.elapsed();
        let total_millis = elapsed.as_millis() as u64;
        let hours = total_millis / 3_600_000;
        let minutes = (total_millis / 60_000) % 60;
        let seconds = (total_millis / 1_000) % 60;
        let millis = total_millis % 1_000;
        let timestamp = format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis);

        let level_label = format!("{:<6}", level.as_str());
        let reset = "\x1b[0m";
        let dim = "\x1b[90m";
        let color = level.color_code();

        println!(
            "{dim}[{timestamp}]{reset} {color}{level_label}{reset} {}",
            message.as_ref()
        );
    }
}
