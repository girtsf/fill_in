use fill_in;

#[test]
fn test() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut input = dir.clone();
    input.push("tests/pyramid_schemes.input");
    let mut expected = dir.clone();
    expected.push("tests/pyramid_schemes.expected");
    let lines = fill_in::load_and_solve(input.to_str().unwrap()).join("\n");
    let expected_lines = std::fs::read_to_string(expected).unwrap();
    assert_eq!(lines.trim(), expected_lines.trim());
}
