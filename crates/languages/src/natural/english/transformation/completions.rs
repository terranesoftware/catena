use catena_suite::traits::transformation::Completions;

pub struct EnglishCompletions;

impl Completions for EnglishCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) {
        
    }
}