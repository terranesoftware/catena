use catena_suite::traits::transformation::Completions;

pub struct PythonCompletions;

impl Completions for PythonCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) {
        
    }
}