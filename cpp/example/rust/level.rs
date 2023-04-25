#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5
}

impl std::fmt::Display for Level {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO ",
            Level::Warn => "WARN ",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL"
        })
    }
}

impl Into<i32> for Level {
    fn into(self) -> i32 {
        self as i32
    }
}

impl Into<u32> for Level {
    fn into(self) -> u32 {
        self as u32
    }
}

impl Into<Level> for i32 {
    fn into(self) -> Level {
        match self {
            0 => Level::Trace,
            1 => Level::Debug,
            2 => Level::Info,
            3 => Level::Warn,
            4 => Level::Error,
            5 => Level::Fatal,
            _ => Level::Info
        }
    }
}

impl Into<Level> for u32 {
    fn into(self) -> Level {
        (self as i32).into()
    }
}