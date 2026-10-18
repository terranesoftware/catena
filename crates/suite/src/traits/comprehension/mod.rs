mod descriptions;
pub use descriptions::Descriptions;

mod diagnostics;
pub use diagnostics::Diagnostics;

pub trait Comprehension {
    type Descriptions: Descriptions;
    type Diagnostics: Diagnostics;
}