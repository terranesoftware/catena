mod descriptor;
use descriptor::RustDescriptor;

mod diagnoser;
use diagnoser::RustDiagnoser;

use catena_suite::traits::Comprehension;

use crate::formal::Rust;

impl Comprehension for Rust {
    type Descriptor = RustDescriptor;
    type Diagnoser = RustDiagnoser;
}