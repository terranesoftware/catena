pub trait Formatter {
    fn formatter(
        &self,
        text: &[u8]
    );
}