#[test]
#[cfg(not(feature = "auto_copy_getters"))]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/allref/01-legacy.rs");
    t.pass("tests/allref/02-simple-single-generic.rs");
    t.pass("tests/allref/03-simple-multi-generic.rs");
    t.pass("tests/allref/04-simple-lifetime-annot.rs");
    t.pass("tests/allref/05-skip-rename-copy-attributes.rs");
    t.pass("tests/allref/06-plays-with-others.rs");
    t.pass("tests/allref/07-dissolve-basic.rs");
    t.pass("tests/allref/08-dissolve-generic-and-ref.rs");
    t.pass("tests/allref/09-dissolve-tuple-struct.rs");
    t.compile_fail("tests/allref/10-dissolve-unit-struct.rs");
}
