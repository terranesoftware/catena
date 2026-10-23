use catena_suite::traits::transformation::{Completer, Edit};

pub struct EnglishCompleter;

impl Completer for EnglishCompleter {
    fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}