use pdf_extract::extract_text;
use std::fs::File;
use std::io::Write;


const DROP_AFTER: &str = "Привязанные \nкарты: Visa Smart Gold (4***********6060)";
const HEADER_COLUMN: &str = "Дата Примечание Сумма \nв \nвалюте \nсчета\n Сумма в \nвалюте \nоперации";

pub fn parse_content_from_filename(filename: &str) -> String {
    let path: String = format!("src/examples/{}.pdf", filename);
    let result: Result<String, pdf_extract::OutputError> = extract_text(path);
    let pdf_content = match result {
        Ok(content) => content,
        Err(err_message) => panic!("Errorrrr: {}", err_message)
    };

    pdf_content
}

pub fn write_output(slice: Vec<&str>) {
    let mut file: File = File::create("output.txt").expect("Failed to create file");

    let mut prev_month: String = String::new();

    for expense in slice {
        let expense: String = expense.replace("\n", "");
        let expense_items: Vec<&str> = expense.split(" ").collect::<Vec<&str>>();
        let money: &str = expense_items[expense_items.len()-4];
        let currency: &str = expense_items[expense_items.len() - 3];
        let date: &str = expense_items[0];
        let text = format!("{};{};{}\n", money, currency, date);
        let splitted_date = date.split(".").collect::<Vec<&str>>();
        let current_month: String = splitted_date[1].to_string();
        let current_year: String = splitted_date[2].to_string();
        if prev_month != current_month {
            prev_month = current_month;
            file.write_all(format!("{}-{}\n", prev_month, current_year).as_bytes()).expect("Failed to write to file");
        }
        file.write_all(text.as_bytes()).expect("Failed to write to file");
    };
}

pub fn split_content(content: &str) -> Vec<&str> {
    let split: std::str::Split<&str> = content.clone().split("\n\n");
    let vec: Vec<&str> = split.collect();
    let index: usize = vec
        .iter()
        .position(|&item| item == DROP_AFTER)
        .unwrap();
    let slice: Vec<&str> = vec[index+1..vec.len()].to_vec()
        .iter()
        .cloned()
        .filter(|&item| {
            item != HEADER_COLUMN && item.len() > 10
        })
        .collect();
    write_output(slice);
    vec
}

fn main() {
    // TODO: make it as a variable from CLI
    let filename: &str = "year";
    let pdf_content = parse_content_from_filename(filename);
    let vec = split_content(&pdf_content);
    vec;
}
