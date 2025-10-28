use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Record {
    from_val: f64,
    from_unit: String,
    to_val: f64,
    to_unit: String,
}

pub fn save(fv: f64, fu: &str, tv: f64, tu: &str) {
    let rec = Record {
        from_val: fv,
        from_unit: fu.to_string(),
        to_val: tv,
        to_unit: tu.to_string(),
    };

    let path = "conversion.json";
    let mut records: Vec<Record> = Vec::new();

    if Path::new(path).exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        if !content.is_empty() {
            records = serde_json::from_str(&content).unwrap_or_default();
        }
    }

    records.push(rec);
    
    let json = serde_json::to_string_pretty(&records).unwrap();
    fs::write(path, json).ok();
}

pub fn show_history() {
    let path = "conversion.json";
    
    if !Path::new(path).exists() {
        println!("\nHistory Konversi:\n\nBelum ada riwayat,silahkan konversi terlebih dahulu.");
        return;
    }

    let content = fs::read_to_string(path).unwrap_or_default();
    if content.is_empty() {
        println!("\nHistory Konversi:\n\nBelum ada riwayat,silahkan konversi terlebih dahulu.");
        return;
    }

    let records: Vec<Record> = serde_json::from_str(&content).unwrap_or_default();

    if records.is_empty() {
        println!("\nRiwayat Konversi:\n\nBelum ada riwayat,silahkan konversi terlebih dahulu.");
        return;
    }

    println!("\nRiwayat Konversi:\n");
    for (i, rec) in records.iter().enumerate() {
        println!("{}. {} {} = {} {}\n", 
            i + 1, 
            rec.from_val, 
            rec.from_unit, 
            rec.to_val, 
            rec.to_unit
        );
    }
}