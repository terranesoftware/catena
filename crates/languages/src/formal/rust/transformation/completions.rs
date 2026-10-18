use catena_suite::traits::transformation::{Completions, Edit};

pub struct RustCompletions;

impl Completions for RustCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}