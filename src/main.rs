use std::{env, fs, path::PathBuf};
mod rummage;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|a| a.as_str()).collect();
    match *args.get(1).expect("Missing first arg") {
        "open" => open(&args[2..]),
        _ => todo!("undone"),
    }
}

fn open(args: &[&str]) {
    if args.is_empty() {
        let path = tj_dir();
        if !path.exists() {
            fs::create_dir(&path).unwrap();
        }
        rummage::edit(path.join(local_now_string()).with_extension("tj")).unwrap();
        return;
    }
    ()
}

fn tj_dir() -> PathBuf {
    dirs::home_dir()
        .expect("unable to determine home directory")
        .join(".tj") //TODO: MAKE STATIC CONSTANT
}

fn local_now_string() -> String {
    chrono::Local::now().format("%d-%m-%Y").to_string()
}
