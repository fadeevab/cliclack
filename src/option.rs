use std::{cell::Cell, marker::PhantomData};

use crate::parse::CliFromStr;

/// Marker trait for `Option<>` types.
#[allow(unused)]
pub trait IsOptionMarker { }

/// Implement `IsOptionMarker` for `Option<T>` where `T` implements `CliFromStr`.
impl<T: CliFromStr> IsOptionMarker for Option<T> { }

/// A helper struct to determine if a type is an `Option<>`.
pub(crate) struct IsOption<'a, T> 
where 
    T: CliFromStr 
{
    is_option: &'a Cell<bool>,
    _marker: PhantomData<T>,
}

impl<'a, T> Clone for IsOption<'a, T> 
where 
    T: CliFromStr 
{
    fn clone(&self) -> Self {
        self.is_option.set(false);
        IsOption {
            is_option: self.is_option,
            _marker: PhantomData,
        }
    }
}

impl<T: IsOptionMarker + CliFromStr> Copy for IsOption<'_, T> {}

/// Determine if a type is an `Option<T: CliFromStr>`.
pub fn is_option<T>() -> bool 
where 
    T: CliFromStr 
{
    let is_option = Cell::new(true);
    let _ = [IsOption::<T> {
        is_option: &is_option,
        _marker: PhantomData,
    }]
    .clone();
    is_option.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_option() {
        assert_eq!(is_option::<Option<i32>>(), true);
        assert_eq!(is_option::<i32>(), false);
    }
}