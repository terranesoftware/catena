use catena_suite::traits::transformation::{Completer, Edit};

pub struct JapaneseCompleter;

impl Completer for JapaneseCompleter {
    fn complete(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}