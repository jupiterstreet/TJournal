use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|a| a.as_str()).collect();
    match *args.get(1).expect("Missing first arg") {
        "open" => open(&args[2..]),
        _ => todo!("undone")
    }
}

fn open(args: &[&str]) {
    if args.is_empty() {
        tj_dir().join(local_now_string());
        return;
    }
    ()
}

fn tj_dir() -> PathBuf {
    dirs::home_dir().expect("unable to determine home directory").join(".tj")
}

fn local_now_string() -> String {
    chrono::Local::now().format("%d-%m-%Y").to_string()
}
