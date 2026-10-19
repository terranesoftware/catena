mod descriptor;
use descriptor::PythonDescriptor;

mod diagnoser;
use diagnoser::PythonDiagnoser;

use catena_suite::traits::Comprehension;

use crate::formal::Python;

impl Comprehension for Python {
    type Descriptor = PythonDescriptor;
    type Diagnoser = PythonDiagnoser;
}