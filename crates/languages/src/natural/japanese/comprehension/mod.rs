mod descriptor;
use descriptor::JapaneseDescriptor;

mod diagnoser;
use diagnoser::JapaneseDiagnoser;

use catena_suite::traits::Comprehension;

use crate::natural::Japanese;

impl Comprehension for Japanese {
    type Descriptor = JapaneseDescriptor;
    type Diagnoser = JapaneseDiagnoser;
}