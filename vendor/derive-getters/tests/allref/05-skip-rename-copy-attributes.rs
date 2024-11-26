use derive_getters::Getters;

#[derive(Getters)]
struct SkipAField {
    keep: u64,

    #[getter(skip)]
    skip: String,
}

impl SkipAField {
    pub fn new<T: Into<String>>(keep: u64, skip: T) -> Self {
        SkipAField { keep, skip: skip.into() }
    }
}

#[derive(Getters)]
struct SkipMany {
    keep1: u64,

    #[getter(skip)]
    skip1: String,

    #[getter(skip)]
    skip2: String,

    #[getter(skip)]
    skip3: String,

    #[getter(skip)]
    skip4: String,

    keep2: String,
}

impl SkipMany {
    pub fn new<T: Into<String>>(
        keep1: u64, skip1: T, skip2: T, skip3: T, skip4: T, keep2: T
    ) -> Self {
        SkipMany {
            keep1,
            skip1: skip1.into(),
            skip2: skip2.into(),
            skip3: skip3.into(),
            skip4: skip4.into(),
            keep2: keep2.into(),
        }
    }
}

#[derive(Getters)]
struct Rename {
    #[getter(rename = "number")]
    field: u64,
}

#[derive(Getters)]
struct RenameMany {
    #[getter(rename = "number1")]
    field1: u64,

    #[getter(rename = "number2")]
    field2: u64,

    field3: u64,

    #[getter(rename = "number3")]
    field4: u64,
}

#[derive(Getters)]
struct CopyMany {
    #[getter(copy)]
    field1: u64,

    #[getter(copy)]
    field2: bool,

    field3: String,
}

#[derive(Getters)]
struct Combination<'a, 'b, 'c, T> {
    #[getter(rename = "skip_me")]
    #[getter(skip)]
    v1: &'a str,

    #[getter(rename = "buffer")]
    v2: &'b [u8],

    #[getter(skip)]
    v3: &'c T,

    #[getter(skip)]
    #[getter(rename = "keep_me")]
    v4: u64,

    #[getter(copy)]
    v5: bool
}

impl<'a, 'b, 'c, T> Combination<'a, 'b, 'c, T> {
    pub fn new(v1: &'a str, v2: &'b [u8], v3: &'c T, v4: u64, v5: bool) -> Self {
        Combination { v1, v2, v3, v4, v5 }
    }
}

#[derive(PartialEq, Eq)]
struct GenericType;

fn main() {
    let s1 = SkipAField::new(45, "You can't get me.");
    assert!(*s1.keep() == 45);

    let s2 = SkipMany::new(33, "Dodge", "Duck", "Dip", "Dive", "...dodge!");
    assert!(*s2.keep1() == 33);
    //assert!(s2.skip1() == "Dodge");
    assert!(s2.keep2() == "...dodge!");

    let s3 = Rename { field: 35 };
    assert!(*s3.number() == 35);

    let s4 = RenameMany { field1: 1, field2: 2, field3: 3, field4: 4 };
    assert!(*s4.number1() == 1);
    assert!(*s4.number2() == 2);
    assert!(*s4.field3() == 3);
    assert!(*s4.number3() == 4);

    let hello = "Hello";
    let s5 = CopyMany { field1: 1, field2: true, field3: hello.to_string() };
    assert!(s5.field1() == 1);
    assert!(s5.field2() == true);
    assert!(s5.field3() == hello);

    let gt = GenericType;
    let buffer: [u8; 12] = [88; 12];
    let s6 = Combination::new("Hello", &buffer, &gt, 64, true);
    //assert!(s5.skip_me() == "Hello");
    assert!(s6.buffer() == &buffer);
    //assert!(s5.v3() == &gt);
    assert!(*s6.keep_me() == 64);
    assert!(s6.v5() == true)
}
