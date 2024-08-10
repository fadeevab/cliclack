use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use crate::CliError;

/// A trait for parsing a type from a string. This is similar to `FromStr`, but
/// is a trait we can implement for types we don't own.
pub trait CliFromStr: Sized {
    fn try_parse(s: &str) -> Result<Self, CliError>;
}

/// Helper macro for implementing [`CliFromStr`] for a list of types that implement
/// [`std::str::FromStr`].
macro_rules! impl_for_fromstr {
    ($($t:ty),*) => {
        $(
            impl CliFromStr for $t {
                fn try_parse(s: &str) -> Result<Self, CliError> {
                    s.parse().map_err(|_| CliError::Parse(s.to_string()))
                }
            }
        )*
    };
}

// Implement `CliFromStr` for all types that implement `FromStr`, which keeps
// backwards compatibility.
impl_for_fromstr!(
    String, 
    usize, 
    isize, 
    u128, 
    u64, 
    u32, 
    u16, 
    u8, 
    i128, 
    i64, 
    i32, 
    i16, 
    i8, 
    f64, 
    f32, 
    bool,
    char,
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddrV4,
    SocketAddrV6
);

/// Implement `CliFromStr` for `Option<T>` where `T` implements `CliFromStr`. This
/// allows us to parse optional values for any type that implements `CliFromStr`.
impl<T: CliFromStr> CliFromStr for Option<T> {
    fn try_parse(s: &str) -> Result<Self, CliError> {
        if s.is_empty() {
            Ok(None)
        } else {
            T::try_parse(s)
                .map_err(|_| CliError::Parse(s.to_string()))
                .map(Some)
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::parse::CliFromStr;

    #[test]
    fn parse_string() {
        assert_eq!(String::try_parse("hello").unwrap(), "hello".to_string());
    }

    #[test]
    fn parse_optional_string() {
        assert_eq!(Option::<String>::try_parse("").unwrap(), None);
        assert_eq!(Option::<String>::try_parse("hello").unwrap(), Some("hello".to_string()));
    }

    #[test]
    fn parse_optional_integer() {
        assert_eq!(Option::<usize>::try_parse("").unwrap(), None);
        assert_eq!(Option::<usize>::try_parse("123").unwrap(), Some(123));
    }
}