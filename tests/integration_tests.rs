use lootifier::Lootifier;
use std::fs;

#[test]
fn test_generate_rules_from_fixture_file() {
    let fixture_path = "tests/fixtures/loadorder.txt";
    let expected_output_path = "tests/fixtures/userlist.yaml";
    
    let lootifier = Lootifier::from_file(fixture_path).unwrap();
    let result = lootifier.generate_rules().unwrap();
    
    let expected_output = fs::read_to_string(expected_output_path).unwrap();
    
    assert_eq!(result.trim(), expected_output.trim());
}

#[test]
fn test_empty_loadorder() {
    let lootifier = Lootifier::from_string("").unwrap();
    let result = lootifier.generate_rules().unwrap();
    
    let expected = r#"groups: []
plugins: []"#;
    
    assert_eq!(result.trim(), expected);
}

#[test]
fn test_comments_only_loadorder() {
    let input = r#"
# Comment line
// Another comment
# Yet another comment
"#;
    
    let lootifier = Lootifier::from_string(input).unwrap();
    let result = lootifier.generate_rules().unwrap();
    
    let expected = r#"groups: []
plugins: []"#;
    
    assert_eq!(result.trim(), expected);
}