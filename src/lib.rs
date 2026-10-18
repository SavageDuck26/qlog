
#[macro_export]
macro_rules! qlog {
    ($path:expr, $content:expr) => {
        {
            use std::io::Write as _;
            let path = $path;
            let path = std::path::Path::new(&path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .expect("qlog: Could not create log folder.");
            }
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("qlog: could not open file");
            writeln!(file, "{}", $content).expect("qlog: Could not write to file.");
        }
    };
}