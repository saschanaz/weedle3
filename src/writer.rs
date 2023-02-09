use crate::Parsed;

pub fn walk_markup<T: MarkupCallback>(parsed: &Parsed, callback: &T) -> T::ReturnType {
    // TODO: Can't write() do the walk?
    // Pass `Vec<dyn Parse>` to callbacks so that they can get the current tree ancestors
    // Both webidl2.js and widlparser support construct-level markup but I don't think neither of ReSpec/bikeshed need it
    // ReSpec: it's used but it doesn't really have to be a block?
    // Let's just defer it as theoretically it's still possible to implement it later
    for def in parsed.definitions.iter() {}
}

// TODO: I don't think this kind of fancy structure can work with Wasm... or can it?
// take a look at wasm-bindgen

pub trait MarkupElement : Sized {
  fn new(s: &str) -> Self;
  fn wrap(items: &[Self]) -> Self;
}

pub trait MarkupCallback {
    type ReturnType: MarkupElement;

    fn identifier(&self) -> Self::ReturnType;
}
