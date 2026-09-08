#[test]
fn backend_specific_and_deprecated_root_aliases_are_private() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/backend-specific-root-aliases.rs");
    cases.compile_fail("tests/ui/backend-specific-referential-aliases.rs");
    cases.compile_fail("tests/ui/deprecated-root-aliases.rs");
    cases.compile_fail("tests/ui/external-structured-line.rs");
}
