//! Public API module.

/// A generic public component.
pub struct Component<T: Clone> {
    pub value: T,
}

/// Return the answer.
pub fn answer<T>(value: T) -> T where T: Clone { value.clone() }

/// A trait with an associated item.
pub trait Render {
    type Output;
    const NAME: &'static str;
    fn render(&self) -> Self::Output;
}

impl<T: Clone> Render for Component<T> {
    type Output = T;
    const NAME: &'static str = "component1";
    fn render(&self) -> Self::Output { self.value.clone() }
}
