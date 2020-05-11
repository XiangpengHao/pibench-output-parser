use pibench_parser;
use std::fs;

#[test]
fn parse_all_no_panic() {
    let all_files = fs::read_dir("tests/fixtures").unwrap();
    for file in all_files {
        let file = file.unwrap().path();
        println!("working on file {}", file.display());
        let contents = fs::read_to_string(file).expect("unable to read file");
        let pibench_obj = pibench_parser::parse_text(&contents);
        assert!(pibench_obj.is_some());
    }
}
