pub fn convert(val: f64, from: &str, to: &str) -> f64 {
    let f = from.to_lowercase();
    let t = to.to_lowercase();
    
    let in_cm = match f.as_str() {
        "cm" => val,
        "inch" => val * 2.54,
        "km" => val * 100000.0,
        "miles" => val * 160934.4,
        _ => val
    };

    match t.as_str() {
        "cm" => in_cm,
        "inch" => in_cm / 2.54,
        "km" => in_cm / 100000.0,
        "miles" => in_cm / 160934.4,
        _ => in_cm
    }
}