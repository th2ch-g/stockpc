
use crate::arg::{*};
use crate::web::{*};
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::process;
use std::path::Path;
use std::fs::OpenOptions;


pub fn make_csv(name: &str, ticker: &str, number: i32, price: i32, path: &str) {

    if Path::new(path).exists() {

        if !check_csv_is_stockpc_file(path) {
            eprintln!("[ERROR] The file specified seems not to be stockpc file");
            process::exit(1);
        }

        let mut file = OpenOptions::new()
            .create(false).write(true).read(true).append(true).open(path).unwrap();

        file.write_all(&format!("{},{},{},{},\n", name, ticker, number, price).as_bytes()).unwrap();

    } else {
        let mut file = File::create(path).unwrap();
        file.write_all("# stockpc\n".as_bytes()).unwrap();
        file.write_all(&format!("{},{},{},{},\n", name, ticker, number, price).as_bytes()).unwrap();

    }

}


pub fn show_csv(path: &str) {

    if !check_csv_is_stockpc_file(path) {
        eprintln!("[ERROR] The file specified seems not to be stockpc file");
        process::exit(1);
    }

    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut line_number = 0;


    println!("{:>10}\t{:>10}\t{:>10}\t{:>10}\t{:>10}\t{:>10}\t{:>10}\t{:>10}", "name", "stock", "number", "purchase", "now", "PASTSUM", "NOWSUM", "CHANGES");

    for line in reader.lines() {
        let line = line.unwrap();
        line_number += 1;

        if line_number == 1 { continue }

        let data: Vec<&str> = line.split(",").collect();

        for i in 0..data.len() {
            if data[i] == "" { continue }
            print!("{:>10}\t", data[i]);
        }
        let number: f32 = data[2].parse().unwrap();
        let purchase: f32 = data[3].parse().unwrap();
        if data.len() > 4 && data[4] != "" {
            let now: f32 = data[4].parse().unwrap();
            print!("{:>10}\t", number * purchase);
            print!("{:>10}\t", number * now);
            print!("{:>10}\t", number * (now - purchase));
        }
        else {
            print!("{:>10}\t", " ");
            print!("{:>10}\t", number * purchase);
        }
        print!("\n");
    }


}


pub fn check_csv(url: &str, path: &str) {

    if !check_csv_is_stockpc_file(path) {
        eprintln!("[ERROR] The file specified seems not to be stockpc file");
        process::exit(1);
    }

    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut line_number = 0;
    let mut data: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        line_number += 1;

        if line_number == 1 { continue }

        let pre_data: Vec<&str> = line.split(",").collect();
        let mut tmp_data: Vec<String> = Vec::new();
        for i in 0..pre_data.len() {
            if pre_data[i] == "" { continue }
            tmp_data.push(pre_data[i].to_string());
        }

        data.push(tmp_data);

    }

    // web scraper
    let new_data = web_scrape(url, data);
    let mut file = File::create(path).unwrap();
    file.write_all("# stockpc\n".as_bytes()).unwrap();

    for i in 0..new_data.len() {
        let mut tmp = String::from("");
        for j in 0..new_data[0].len() {
            tmp += &new_data[i][j];
            tmp += ",";
        }
        tmp += "\n";
        file.write_all(&tmp.as_bytes()).unwrap();
    }

}


pub fn rm_csv(name: &str, path: &str) {

    if !check_csv_is_stockpc_file(path) {
        eprintln!("[ERROR] The file specified seems not to be stockpc file");
        process::exit(1);
    }

    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut line_number = 0;
    let mut hit_flag = false;
    let mut data = String::from("");

    for line in reader.lines() {
        let line = line.unwrap();
        line_number += 1;

        if line_number == 1 { continue }

        if line.contains(&format!("{}{}", name, ",")) {
            hit_flag = true;
        }
        else {
            data += &line;
            data += "\n";
        }

    }

    if !hit_flag {
        eprintln!("[ERROR] ticker is not found");
        process::exit(1);
    }

    let mut file = File::create(path).unwrap();
    file.write_all("# stockpc\n".as_bytes()).unwrap();
    file.write_all(&data.as_bytes()).unwrap();

}


pub fn reset_csv(path: &str) {

    if !check_csv_is_stockpc_file(path) {
        eprintln!("[ERROR] The file specified seems not to be stockpc file");
        process::exit(1);
    }

    let reset_csv_result = fs::remove_file(&path);

    match reset_csv_result {
        Ok(_) => { () },
        Err(_) => {
            eprintln!("[ERROR] Failed to reset stockpc file : {}", &path);
            process::exit(1);
        }
    }

}

fn check_csv_is_stockpc_file(path: &str) -> bool {

    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut line_number = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        line_number += 1;
        if line_number == 1 {
            if line.contains("# stockpc") {
                return true;
            }

        }
        else { break }
    }
    false
}
