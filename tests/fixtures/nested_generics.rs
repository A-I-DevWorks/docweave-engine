//! Fixture for file-level documentation.

/// Outer module docs.
#[allow(dead_code)]
pub mod outer {
    //! Inline module documentation.

    /// Generic struct docs.
    #[derive(Debug)]
    pub struct Record<T> {
        pub value: T,
    }

    /// Generic trait docs.
    pub trait Convert<T> {
        fn convert(&self) -> T;
    }

    impl<T> Record<T> {
        /// Method docs.
        pub fn value(&self) -> &T {
            &self.value
        }
    }

    /// Generic enum docs.
    pub enum Choice<T> {
        One(T),
        None,
    }

    /// Constant docs.
    pub const ANSWER: usize = 42;

    /// Generic function docs.
    pub fn make<T>(value: T) -> Record<T> {
        Record { value }
    }
}
