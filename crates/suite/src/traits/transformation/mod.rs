mod completer;
pub use completer::Completer;

mod edit;
pub use edit::Edit;

mod formatter;
pub use formatter::Formatter;

mod propagator;
pub use propagator::Propagator;

mod suggester;
pub use suggester::Suggester;

pub trait Transformation {
    type Completer: Completer;
    type Formatter: Formatter;
    type Propagator: Propagator;
    type Suggester: Suggester;
}