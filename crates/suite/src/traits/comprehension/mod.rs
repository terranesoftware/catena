pub mod descriptor;
pub use descriptor::Descriptor;

pub mod diagnoser;
pub use diagnoser::Diagnoser;

pub trait Comprehension {
    type Descriptor: Descriptor;
    type Diagnoser: Diagnoser;
}