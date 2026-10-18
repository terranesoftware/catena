mod descriptions;
pub use descriptions::Descriptions;

mod diagnostics;
pub use diagnostics::Diagnostics;

mod highlights;
pub use highlights::Highlights;

pub trait Comprehension {
    type Descriptions: Descriptions;
    type Diagnostics: Diagnostics;
    type Highlights: Highlights;
}