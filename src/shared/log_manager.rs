use env_logger::Builder;
use log::{Level, LevelFilter};
use std::{
    collections::HashSet,
    io::Write,
};

pub fn set_logging(log_levels: &[Level]) {

    let mut builder = Builder::new();

    // Create a HashSet for filtering only the selected log levels
    let selected_levels: HashSet<Level> = log_levels.iter().cloned().collect();

    builder
        .filter(None, LevelFilter::Trace) // Enable all levels up to Trace, but filter in the format function
        .format(move |buf, record| {
            if selected_levels.contains(&record.level()) {
                writeln!(
                    buf,
                    "{} [{}] - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            } else {
                Ok(()) // Skip levels not in the selected set
            }
        })
        .init();
}
