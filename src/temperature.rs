pub fn convert(val: f64, from: &str, to: &str) -> f64 {
    let f = from.to_lowercase();
    let t = to.to_lowercase();
    
    let celsius = match f.as_str() {
        "celsius" => val,
        "fahrenheit" => (val - 32.0) * 5.0 / 9.0,
        "kelvin" => val - 273.15,
        _ => val
    };

    match t.as_str() {
        "celsius" => celsius,
        "fahrenheit" => celsius * 9.0/5.0 + 32.0,
        "kelvin" => celsius + 273.15,
        _ => celsius
    }
}