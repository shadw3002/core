use log::{Record, Level, Metadata, LevelFilter, Log};

pub fn init() {
    // TODO: level control
    lazy_static!{
        static ref LOGGER: Logger = Logger::new(Level::Info);
    }
    log::set_logger(&*LOGGER).err().map(|err| panic!("set logger: {}", err));
    log::set_max_level(LevelFilter::Info); 
    info!("[logger] init done.");
}

#[repr(u8)]
#[allow(dead_code)]
enum ColorCode {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

struct Logger{
    level: Level
}

impl Logger {
    fn new(level: Level) -> Logger {
        Self {level}
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return
        }
        let color = match record.level() {
            Level::Error => ColorCode::Red         ,
            Level::Warn  => ColorCode::BrightYellow,
            Level::Info  => ColorCode::Blue        ,
            Level::Debug => ColorCode::Green       ,
            Level::Trace => ColorCode::BrightBlack ,
        };
        let cpu_id = crate::cpu::id();
        println!(
            "\x1b[{}m[{}][{}] {}\x1b[0m", 
            color as u8, 
            cpu_id, 
            record.level().as_str(), 
            record.args()
        );
    }
    fn flush(&self) {}
}