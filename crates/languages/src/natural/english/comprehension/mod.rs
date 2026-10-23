mod descriptor;
use descriptor::EnglishDescriptor;

mod diagnoser;
use diagnoser::EnglishDiagnoser;

use catena_suite::traits::Comprehension;

use crate::natural::English;

impl Comprehension for English {
    type Descriptor = EnglishDescriptor;
    type Diagnoser = EnglishDiagnoser;
}