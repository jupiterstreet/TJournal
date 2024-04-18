use std::{
    env, fs,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;
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
        open_for_day(local_now_string());
        return;
    }
    if let Some(arg) = args.get(0) {
        let date = date_format(arg).unwrap().format("%d-%m-%Y").to_string();
        open_for_day(date);
    }
    ()
}

fn date_format(date: &str) -> Result<NaiveDate, &str> {
    let date: Vec<&str> = date.split(&['/', '-', ' ', '_']).collect();
    if date.len() != 3 {
        return Err("Date is not of the format dd/mm/yyyy");
    }
    chrono::NaiveDate::from_ymd_opt(
        date.get(2)
            .unwrap()
            .parse::<i32>()
            .or_else(|_| Err("Year was not a number"))?,
        date.get(1)
            .unwrap()
            .parse::<u32>()
            .or_else(|_| Err("Month was not a number"))?,
        date.get(0)
            .unwrap()
            .parse::<u32>()
            .or_else(|_| Err("Day was not a number"))?,
    )
    .ok_or_else(|| "Date does not fall in the required bounds")
}

fn open_for_day<P: AsRef<Path>>(path: P) {
    let dir = tj_dir();
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    rummage::edit(dir.join(path.as_ref()).with_extension("tj")).unwrap();
}

fn tj_dir() -> PathBuf {
    dirs::home_dir()
        .expect("unable to determine home directory")
        .join(".tj") //TODO: MAKE STATIC CONSTANT
}

fn local_now_string() -> String {
    chrono::Local::now().format("%d-%m-%Y").to_string()
}
