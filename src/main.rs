use std::{
    env::{self}, fs,
    path::PathBuf,
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
    let starts_with_date = args.is_empty() || !args.get(0).unwrap().starts_with('@');

    let date = if starts_with_date {
        chrono::Local::now().date_naive()
    } else {
        parse_date(args.get(0).unwrap())
            .unwrap()
    };

    let mut filename = format_date_for_file_name(&date);

    let tags = if starts_with_date && !args.is_empty() {
        &args[1..]
    }
    else {
        args
    };

    tags.iter().for_each(|tag| {
        filename.push('_');
        filename.push_str(tag);
    });

    open_for_filename(filename, date, tags);
}

fn parse_date(date: &str) -> Result<NaiveDate, &str> {
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

fn open_for_filename<S: AsRef<str>>(filename: S, date: NaiveDate, tags: &[&str]) {
    let dir = tj_dir();
    if !dir.exists() {
        fs::create_dir(&dir).unwrap();
    }
    // todo, work on the FnOnce
    rummage::edit(dir.join(filename.as_ref()).with_extension("tj"), || default_entry(date, tags)).unwrap();
}

fn tj_dir() -> PathBuf {
    dirs::home_dir()
        .expect("unable to determine home directory")
        .join(".tj") //TODO: MAKE STATIC CONSTANT
}

fn format_date_for_file_name(date: &NaiveDate) -> String {
    date.format("%d-%m-%Y").to_string()
}

fn default_entry(date: NaiveDate, tags: &[&str]) -> String {
    let date_str = date.format("%A %v");
    return format!("TAGS: {}\n{date_str}", tags.join(" "));
}
