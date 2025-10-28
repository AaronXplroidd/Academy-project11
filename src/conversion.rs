use crate::temperature;
use crate::length;
use crate::history;

#[derive(Debug)]
pub enum UnitType {
    Temp,
    Len,
}

pub fn handle_convert(from: String, to: String, val: f64) {
    let from_type = get_unit_type(&from);
    let to_type = get_unit_type(&to);

    if from_type.is_none() {
        eprintln!("Error: [ERROR] Satuan asal '{}' tidak dikenali.", from);
        std::process::exit(1);
    }
    
    if to_type.is_none() {
        eprintln!("Error: [ERROR] Satuan tujuan '{}' tidak dikenali.", to);
        std::process::exit(1);
    }

    let ft = from_type.unwrap();
    let tt = to_type.unwrap();

    match (&ft, &tt) {
        (UnitType::Temp, UnitType::Temp) => {
            let res = temperature::convert(val, &from, &to);
            let fsym = get_sym(&from);
            let tsym = get_sym(&to);
            println!("{} {} = {} {}", val, fsym, res, tsym);
            history::save(val, &fsym, res, &tsym);
        }
        (UnitType::Len, UnitType::Len) => {
            let res = length::convert(val, &from, &to);
            let fsym = get_sym(&from);
            let tsym = get_sym(&to);
            println!("{} {} = {} {}", val, fsym, res, tsym);
            history::save(val, &fsym, res, &tsym);
        }
        _ => {
            let fcat = type_to_str(&ft);
            let tcat = type_to_str(&tt);
            eprintln!("Error: [ERROR] Tidak dapat mengonversi satuan yang berbeda kategori: [{}] {} → [{}] {}", 
                fcat, from, tcat, to);
            std::process::exit(1);
        }
    }
}

fn get_unit_type(unit: &str) -> Option<UnitType> {
    let u = unit.to_lowercase();
    match u.as_str() {
        "celsius" | "fahrenheit" | "kelvin" => Some(UnitType::Temp),
        "cm" | "inci" | "km" | "mil" => Some(UnitType::Len),
        _ => None
    }
}

fn type_to_str(t: &UnitType) -> &str {
    match t {
        UnitType::Temp => "suhu",
        UnitType::Len => "panjang",
    }
}

fn get_sym(unit: &str) -> String {
    let u = unit.to_lowercase();
    match u.as_str() {
        "celsius" => "°C".to_string(),
        "fahrenheit" => "°F".to_string(),
        "kelvin" => "K".to_string(),
        _ => u
    }
}

pub fn list_units() {
    println!("\nSatuan yang didukung:\n");
    
    let units = vec![
        ("suhu", "celsius"),
        ("suhu", "fahrenheit"),
        ("suhu", "kelvin"),
        ("panjang", "cm"),
        ("panjang", "inci"),
        ("panjang", "km"),
        ("panjang", "mil"),
    ];

    let mut idx = 1;
    for (cat, unit) in units {
        println!("{}. [{}] {}\n", idx, cat, unit);
        idx += 1;
    }
}