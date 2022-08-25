pub type TextSpan = [[u64; 2]; 2];
pub trait Location {
    fn get_range(&self) -> TextSpan;
}
