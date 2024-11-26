//! Try with generics and references.

use derive_getters::Dissolve;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Dissolve)]
struct SimpleUnnamed(u64, i64);

#[derive(Dissolve)]
struct MultiAnnotated<'a, 'b, 'c, T>(&'a str, &'b [u8], &'c T, String);

#[derive(Dissolve)]
#[dissolve(rename = "unmake")]
struct PolyAnnotated<'a, 'b, 'c, T>(&'a str, &'b [u8], &'c T, String);

fn main() {
    let buffer: [u8; 12] = [88; 12];
    
    let su = SimpleUnnamed(44,-100);
    let (a, b) = su.dissolve();

    assert_eq!(44, a);
    assert_eq!(-100, b);

    let ma = MultiAnnotated("Hi", &buffer, &su, "Another".to_owned());

    let (v1, v2, v3, owned) = ma.dissolve();
    
    assert!(v1 == "Hi");
    assert!(v2 == &buffer);
    assert!(*v3 == su);
    assert!(owned == "Another");

    let pa = PolyAnnotated("Hi", &buffer, &su, "Another".to_owned());
    let (v1, v2, v3, owned) = pa.unmake();
    assert!(v1 == "Hi");
    assert!(v2 == &buffer);
    assert!(*v3 == su);
    assert!(owned == "Another");
}
