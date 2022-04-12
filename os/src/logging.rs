use log::{Record, Level, Metadata, LevelFilter, Log};

lazy_static! {
    static ref LOGGER: Logger = Logger::new(Level::Info);
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
        let (color, label) = match record.level() {
            Level::Error => ("31", "Error"),
            Level::Warn => ("93", "Warn"),
            Level::Info => ("34", "Info"),
            Level::Debug => ("32", "Debug"),
            Level::Trace => ("90", "Trace"),
        };
        let cpu_id = crate::cpu::id();
        println!("\x1b[{}m[{}][{}] {}\x1b[0m", color, cpu_id, label, record.args());
    }
    fn flush(&self) {}
}

// spin once
pub fn init() {
    static LOGGER_INIT: spin::Once<()> = spin::Once::new();
    LOGGER_INIT.call_once(||{
        // TODO: level control
        log::set_logger(&*LOGGER).err().map(|err| panic!("set logger: {}", err));
        log::set_max_level(LevelFilter::Info); 
        info!("logger init done: {:#x}", &LOGGER as *const _ as usize);
    });
}
