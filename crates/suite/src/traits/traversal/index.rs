pub trait Index {
    fn index(
        &self,
        text: &[u8]
    );
}