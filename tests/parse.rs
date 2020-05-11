use pibench_parser::PiBenchData;
use std::fs;

#[test]
fn parse_all_no_panic() {
    let all_files = fs::read_dir("tests/fixtures").unwrap();
    for file in all_files {
        let file = file.unwrap().path();
        println!("working on file {}", file.display());
        let contents = fs::read_to_string(file).expect("unable to read file");
        let pibench_obj = PiBenchData::from_text(&contents);
        assert!(pibench_obj.is_some());
    }
}

#[test]
fn parse_one_json() {
    let file = fs::read_dir("tests/fixtures")
        .unwrap()
        .nth(0)
        .unwrap()
        .unwrap()
        .path();
    let content = fs::read_to_string(file).expect("unable to read file");
    let pibench_obj = PiBenchData::from_text(&content);
    assert!(pibench_obj.is_some());
    println!("{}", pibench_obj.unwrap().to_json());
}
