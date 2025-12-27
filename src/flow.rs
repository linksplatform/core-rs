use std::ops::{ControlFlow, FromResidual, Try};

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
}

impl FromResidual for Flow {
    fn from_residual(_: <Self as Try>::Residual) -> Self {
        Flow::Break
    }
}

impl Try for Flow {
    type Output = ();
    type Residual = Flow;

    fn from_output(_: Self::Output) -> Self {
        Flow::Continue
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Flow::Continue => ControlFlow::Continue(()),
            Flow::Break => ControlFlow::Break(Flow::Break),
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
