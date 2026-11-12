use crate::core::Ref;

/// Extension trait borrowing any generator or shape via [`Ref`] without consuming it.
pub trait ByRef: Sized {
    /// Returns a `Copy` reference wrapper forwarding generator and spatial traits.
    fn by_ref(&self) -> Ref<'_, Self> {
        Ref::new(self)
    }
}

impl<B: Sized> ByRef for B {}
