use catena_suite::traits::transformation::{Completer, Edit};

pub struct RustCompleter;

impl Completer for RustCompleter {
    fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}