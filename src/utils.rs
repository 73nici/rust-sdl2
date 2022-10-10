pub fn log_and_exit(error: &str, code: Option<i32>) {
    println!("{}", error);

    match code {
        None => { std::process::exit(1) },
        Some(code) => { std::process::exit(code) }
    }
}