use crate::core::Ref;

pub trait Base: Sized {
    fn as_ref(&self) -> Ref<Self> {
        Ref::new(self)
    }
}

impl<B: Sized> Base for B {}
