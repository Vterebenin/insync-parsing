use pdf_extract::extract_text;

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

pub fn split_content(content: &str) -> Vec<&str> {
    let split: std::str::Split<&str> = content.split("\n\n");
    let vec: Vec<&str> = split.collect::<Vec<&str>>();
    let index: usize = vec
        .iter()
        .position(|&item| item == DROP_AFTER)
        .unwrap();
    println!("{}", index);

    let mut slice: Vec<&str> = vec[index+1..vec.len()].to_vec();
    let slice: Vec<&str> = slice
        .iter()
        .cloned()
        .filter(|&item| item != HEADER_COLUMN)
        .collect();

    println!("{:#?}", slice);
    vec
}

fn main() {
    // TODO: make it as a variable from CLI
    let filename: &str = "year";
    let pdf_content = parse_content_from_filename(filename);
    let vec = split_content(&pdf_content);
    vec;
}
