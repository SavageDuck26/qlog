
#[macro_export]
macro_rules! qlog {
    ($path:expr, $content:expr) => {
        {
            use std::io::Write as _;
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open($path)
                .expect("qlog: could not open file");
            writeln!(file, "{}", $content).expect("qlog: could not write to file");
        }
    };
}

