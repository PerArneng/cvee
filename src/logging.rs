use fern::Dispatch;
use log::LevelFilter;
use colored::*;
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the logger
    Dispatch::new()
        .format(|out, message, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".magenta(),
            };
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                level,
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}