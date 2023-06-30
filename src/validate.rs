pub trait Validate<T> {
    type Err;

    fn validate(&self, input: &T) -> Result<(), Self::Err>;
}

impl<T, F, E> Validate<T> for F
where
    F: Fn(&T) -> Result<(), E>,
{
    type Err = E;

    fn validate(&self, input: &T) -> Result<(), Self::Err> {
        self(input)
    }
}
