use crate::Generator;

#[derive(Clone)]
pub struct Constant<T: Copy + Clone> {
    pub value: T,
}

impl<T: Copy + Clone> Constant<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Copy + Clone> Generator<T> for Constant<T> {
    type Output = T;

    fn generate(&self, _input: T) -> T {
        self.value
    }
}
