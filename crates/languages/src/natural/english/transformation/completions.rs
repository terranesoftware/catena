use catena_suite::traits::transformation::{Completions, Edit};

pub struct EnglishCompletions;

impl Completions for EnglishCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}