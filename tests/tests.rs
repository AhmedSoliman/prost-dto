#[test]
fn dto_tests() {
    let t = trybuild::TestCases::new();
    t.pass("./tests/simple-into-prost-enum.rs");
    t.pass("./tests/into-prost-enum-unit.rs");
    t.pass("./tests/simple-into-prost-struct.rs");
    t.pass("./tests/from-prost-enum-unit.rs");
    t.pass("./tests/simple-from-prost-struct.rs");
}
