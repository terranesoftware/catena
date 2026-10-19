use catena_suite::traits::transformation::{Completer, Edit};

pub struct PythonCompleter;

impl Completer for PythonCompleter {
    fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}