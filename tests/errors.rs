#[test]
pub fn error_diagnostics() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/errors/*.rs");
}
