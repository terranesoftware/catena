use catena_suite::traits::transformation::Completions;

pub struct RustCompletions;

impl Completions for RustCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) {
        
    }
}