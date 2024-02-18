use std::io;

pub mod num;
pub mod string;
pub mod iters;
pub mod varint;
pub mod other;

pub trait Convert {
    ///
    /// Convert to bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    ///
    /// async fn main()  {
    ///     let data = "HelloWorld".to_string();
    ///     let mut res = vec![];
    ///     data.to_bytes(&mut res);
    /// }
    /// ```
    fn to_bytes(&self, tx: &mut Vec<u8>);

    ///
    /// Convert from bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use naumi::Coder;
    /// use naumi::types::Convert;
    ///
    /// async fn main() {
    ///     let data = "HelloWorld".to_string();
    ///     let mut res = vec![];
    ///     data.to_bytes(&mut res);
    ///
    ///     assert_eq!(data, String::from_bytes(&mut res).unwrap());
    /// }
    /// ```
    fn from_bytes(rx: &mut Vec<u8>) -> io::Result<Self> where Self: Sized;
}