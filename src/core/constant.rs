use crate::Generator;

#[derive(Clone)]
pub struct Constant<T: Copy> {
    pub value: T,
}

impl<T: Copy> Constant<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Copy> Generator<T> for Constant<T> {
    type Output = T;

    fn generate(&self, _input: T) -> T {
        self.value
    }
}
