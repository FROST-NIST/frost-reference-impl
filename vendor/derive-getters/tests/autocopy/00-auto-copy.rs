use derive_getters::Getters;

#[derive(Getters)]
pub struct Auto {
    num: u32,
    string: String,
}

fn main() {
    let num = 0;
    let string = "Hello".to_string();
    let auto = Auto {
        num,
        string: string.clone(),
    };

    assert_eq!(num, auto.num()); // getter returns a copy
    assert_eq!(&string, auto.string()); // getter returns a reference
}
