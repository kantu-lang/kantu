#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TypePositivityValidated<T>(T);

impl<T> TypePositivityValidated<T> {
    pub fn unchecked_new(value: T) -> Self {
        Self(value)
    }
}

impl<T> TypePositivityValidated<T> {
    pub fn raw(self) -> T {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypePositivityError {
    DummyVariantToPreventUninhabitedWarnings,
}
