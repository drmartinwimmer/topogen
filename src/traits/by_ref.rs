use crate::core::Ref;

pub trait ByRef: Sized {
    fn by_ref(&self) -> Ref<'_, Self> {
        Ref::new(self)
    }
}

impl<B: Sized> ByRef for B {}
