use chrono;
use fern::colors::{ColoredLevelConfig, Color};

pub(crate) fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .warn(Color::Yellow)
        .error(Color::Red)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightMagenta);
    fern::Dispatch::new().format(move |out, message, record| {
        out.finish(format_args!(
            "[{}][{}][{}]{}{}{}",
            chrono::offset::Local::now().time(),
            record.target().split("::").next().unwrap(),
            colors.color(record.level()),
            format_args!(
                "\x1B[{}m",
                colors.get_color(&record.level()).to_fg_str()
            ),
            message,
            "\x1B[0m",
        ))
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
    .chain(fern::log_file("output.log")?)
    .apply()?;
    Ok(())
}