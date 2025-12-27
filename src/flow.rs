use std::ops::ControlFlow;

/// Represents the control flow of an operation, similar to `ControlFlow`.
///
/// This is a simplified enum that can be used with iterators and callbacks
/// to indicate whether to continue or break early.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flow {
    Continue,
    Break,
}

impl Flow {
    /// Returns `true` if this is `Flow::Continue`.
    pub fn is_continue(&self) -> bool {
        matches!(self, Flow::Continue)
    }

    /// Returns `true` if this is `Flow::Break`.
    pub fn is_break(&self) -> bool {
        matches!(self, Flow::Break)
    }

    /// Converts this Flow into a ControlFlow that can be used with try_for_each.
    ///
    /// This method enables using Flow with iterator methods like `try_for_each`:
    ///
    /// ```
    /// use platform_data::Flow;
    /// use std::ops::ControlFlow;
    ///
    /// let mut count = 0;
    /// let result = (0..10).try_for_each(|i| {
    ///     count += 1;
    ///     if i == 5 { Flow::Break.into_control_flow() } else { Flow::Continue.into_control_flow() }
    /// });
    /// assert_eq!(count, 6);
    /// ```
    pub fn into_control_flow(self) -> ControlFlow<()> {
        match self {
            Flow::Continue => ControlFlow::Continue(()),
            Flow::Break => ControlFlow::Break(()),
        }
    }
}

impl<C, B> From<ControlFlow<C, B>> for Flow {
    fn from(flow: ControlFlow<C, B>) -> Self {
        match flow {
            ControlFlow::Continue(_) => Flow::Continue,
            ControlFlow::Break(_) => Flow::Break,
        }
    }
}

impl From<Flow> for ControlFlow<()> {
    fn from(flow: Flow) -> Self {
        match flow {
            Flow::Continue => ControlFlow::Continue(()),
            Flow::Break => ControlFlow::Break(()),
        }
    }
}
