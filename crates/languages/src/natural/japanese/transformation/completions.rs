use catena_suite::traits::transformation::{Completions, Edit};

pub struct JapaneseCompletions;

impl Completions for JapaneseCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}