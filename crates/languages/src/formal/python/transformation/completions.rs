use catena_suite::traits::transformation::{Completions, Edit};

pub struct PythonCompletions;

impl Completions for PythonCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}