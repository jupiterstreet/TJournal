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
    open_for_day(if args.is_empty() {
        local_now_string()
    } else {
        let mut iter = args.into_iter();
        let mut res = if !args.get(0).unwrap().starts_with('@') {
            date_format(iter.next().unwrap())
                .unwrap()
                .format("%d-%m-%Y")
                .to_string()
        } else {
            local_now_string()
        };

        while let Some(tag) = iter.next() {
            if !tag.starts_with('@') {
                panic!("Unknown argument")
            }
            res.push_str(format!("_{}", &tag[1..]).as_str())
        }

        res
    })
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
