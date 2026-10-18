use catena_suite::traits::transformation::Completions;

pub struct JapaneseCompletions;

impl Completions for JapaneseCompletions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    ) {
        
    }
}