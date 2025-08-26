use derive_getters::Dissolve;

#[derive(Debug, Copy, Clone, Dissolve)]
struct Unit;

fn main() {
    let _ = Unit;
}
