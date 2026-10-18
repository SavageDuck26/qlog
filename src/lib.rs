#[macro_export]
macro_rules! qlog {
    ($path:expr, $($arg:tt)*) => {
        $crate::qlog_write(std::path::Path::new(&$path), std::format_args!($($arg)*))
    };
}

#[doc(hidden)]
pub fn qlog_write(path: &std::path::Path, line: std::fmt::Arguments<'_>) -> std::io::Result<()> {
    use std::io::Write as _;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| qlog_step("could not create log folder", err))?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|err| qlog_step("could not open file", err))?;
    writeln!(file, "{line}").map_err(|err| qlog_step("could not write to file", err))?;
    Ok(())
}

fn qlog_step(what: &str, err: std::io::Error) -> std::io::Error {
    std::io::Error::new(err.kind(), format!("qlog: {what}: {err}"))
}
