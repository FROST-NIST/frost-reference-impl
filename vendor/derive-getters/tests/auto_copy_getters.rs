#[test]
#[cfg(feature = "auto_copy_getters")]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/autocopy/00-auto-copy.rs");
    t.pass("tests/autocopy/01-legacy.rs");
    t.pass("tests/autocopy/02-simple-single-generic.rs");
    t.pass("tests/autocopy/03-simple-multi-generic.rs");
    t.pass("tests/autocopy/04-simple-lifetime-annot.rs");
    t.pass("tests/autocopy/05-skip-rename-copy-attributes.rs");
    t.pass("tests/autocopy/06-plays-with-others.rs");
    t.pass("tests/autocopy/07-dissolve-basic.rs");
    t.pass("tests/autocopy/08-dissolve-generic-and-ref.rs");
    t.pass("tests/autocopy/09-dissolve-tuple-struct.rs");
    t.compile_fail("tests/autocopy/10-dissolve-unit-struct.rs");
}
