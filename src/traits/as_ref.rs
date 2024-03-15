use crate::core::Ref;

pub trait AsRef: Sized {
    fn as_ref(&self) -> Ref<Self> {
        Ref::new(self)
    }
}

impl<B: Sized> AsRef for B {}
