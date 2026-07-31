use std::collections::HashMap;
use std::fs;

pub fn parse_csv(content: &str) -> (Vec<String>, Vec<HashMap<String, String>>) {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return (vec![], vec![]);
    }

    let headers: Vec<String> = lines[0].split(',').map(|s| s.trim().to_string()).collect();
    let mut records = Vec::new();

    for line in &lines[1..] {
        if line.trim().is_empty() { continue; }
        let values: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        let mut map = HashMap::new();
        for (i, header) in headers.iter().enumerate() {
            if let Some(val) = values.get(i) {
                map.insert(header.clone(), val.to_string());
            }
        }
        records.push(map);
    }

    (headers, records)
}

pub fn column_average(records: &[HashMap<String, String>], column: &str) -> Option<f64> {
    let mut sum = 0.0;
    let mut count = 0;

    for record in records {
        if let Some(val_str) = record.get(column) {
            if let Ok(val) = val_str.parse::<f64>() {
                sum += val;
                count += 1;
            }
        }
    }

    if count > 0 { Some(sum / count as f64) } else { None }
}

pub fn run() {
    println!("=== LAB 4 STRETCH GOAL ===");
    let csv_data = "Name,Department,Score\nAnika,CPE,95\nBob,EEE,80\nCharlie,CPE,88";
    let file_path = "sample.csv";
    fs::write(file_path, csv_data).expect("Unable to write file");

    let content = fs::read_to_string(file_path).unwrap();
    let (_headers, records) = parse_csv(&content);

    // Filter rows
    let cpe_students: Vec<&HashMap<String, String>> = records
        .iter()
        .filter(|r| r.get("Department").map_or(false, |d| d == "CPE"))
        .collect();

    println!("CPE Department Records: {:?}", cpe_students);

    if let Some(avg) = column_average(&records, "Score") {
        println!("Average Score: {:.2}", avg);
    }

    let _ = fs::remove_file(file_path);
}