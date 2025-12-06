use std::{borrow::Borrow, marker::PhantomData};

#[allow(private_bounds)]
pub struct CorrectResultTest<'s, Parse, Solve, T, I, O>
where
    Parse: ParserOrNone<'s, T>,
    I: ?Sized,
    T: ?Sized,
    O: 'static,
{
    pub parser: Parse,
    pub solver: Solve,
    pub example: &'s T,
    pub result: &'static O,
    pub marker: PhantomData<I>,
}
pub trait Unindentable {
    type Output: Borrow<Self>;
    fn unindent(&self) -> Self::Output;
}

impl Unindentable for str {
    type Output = String;
    fn unindent(&self) -> String {
        unindent::unindent(self)
    }
}

impl Unindentable for [u8] {
    type Output = Vec<u8>;
    fn unindent(&self) -> Vec<u8> {
        unindent::unindent_bytes(self)
    }
}

trait ParserOrNone<'s, T: ?Sized> {
    type Parsed;
    fn parse(self, input: &'s T) -> Self::Parsed;
}

impl<'s, F: FnOnce(&T) -> I, T: Unindentable + ?Sized + 's, I> ParserOrNone<'s, T> for F {
    type Parsed = I;
    fn parse(self, input: &'s T) -> Self::Parsed {
        self(input)
    }
}

impl<'s, T: Unindentable + ?Sized + 's> ParserOrNone<'s, T> for Option<()> {
    type Parsed = &'s T;
    fn parse(self, input: &'s T) -> Self::Parsed {
        match self {
            Some(()) => panic!("parser should be a function or None"),
            None => input,
        }
    }
}

#[allow(private_bounds)]
impl<'s, Parse, Solve, T, I, O, Solution> CorrectResultTest<'s, Parse, Solve, T, I, O>
where
    Parse: ParserOrNone<'s, T>,
    Solve: FnOnce(&I) -> Solution,
    Parse::Parsed: Borrow<I>,
    T: ?Sized,
    I: ?Sized,
    Solution: std::cmp::PartialEq<O> + std::fmt::Debug + 'static,
    O: std::fmt::Debug + 'static,
{
    #[cfg_attr(not(test), allow(unused))]
    #[allow(clippy::missing_panics_doc)]
    pub fn test(self) {
        assert_eq!(
            &(self.solver)(self.parser.parse(self.example).borrow()),
            self.result
        );
    }
}

#[macro_export]
macro_rules! example_tests {
    // Note: the syntax has changed a little bit since previous versions, in
    // order to enable specifying a per-part parser in a kinda ergonomic way. In
    // particular, the parser is always specified before the per-part example
    // data, but *after* the global example data. Make sure to check the order
    // of the parameters.
    (
        $example_data:expr,
        $(
            parser: $per_part_parser:expr,
            $($per_part_example_data:literal,)?
            $solver_name:ident => $result:expr
        ),+
        $(,)?
    ) => {
        #[cfg(test)]
        mod example_tests {
            $(
                #[test]
                fn $solver_name() {
                    use std::borrow::Borrow;
                    use $crate::testing::{CorrectResultTest, Unindentable};
                    let parser = $per_part_parser;
                    #[allow(unused_variables)]
                    let example_data = $example_data.unindent();
                    $(
                        let example_data = $per_part_example_data.unindent();
                    )?
                    {
                    CorrectResultTest {
                        parser,
                        solver: super::$solver_name,
                        example: example_data.borrow(),
                        result: &$result,
                        marker: std::marker::PhantomData,
                    }.test();
                }
                }
            )*
        }
    };
    ($example_data:expr, $($solver_name:ident => $result:expr),+ $(,)?) => {
        example_tests! {
            $example_data,
            $(
                parser: super::parse,
                $solver_name => $result
            ),*
        }
    };
}

#[macro_export]
macro_rules! known_input_tests {
    (
        input: $input:expr,
        $(
            parser: $per_part_parser:expr,
            $solver_name:ident => $result:expr
        ),+
        $(,)?
    ) => {
        #[cfg(test)]
        mod known_input_tests {
            $(
                #[test]
                fn $solver_name() {
                    use std::borrow::Borrow;
                    use $crate::testing::{CorrectResultTest, Unindentable};
                    #[allow(unused_variables)]
                    let parser = $per_part_parser;
                    let example_data = $input.unindent();
                    {
                    CorrectResultTest {
                        parser,
                        solver: super::$solver_name,
                        example: example_data.borrow(),
                        result: &$result,
                        marker: std::marker::PhantomData,
                    }.test();
                }
                }
            )*
        }
    };
    (input: $input:expr, $($solver_name:ident => $result:expr),+ $(,)?) => {
        known_input_tests! {
            input: $input,
            $(
                parser: super::parse,
                $solver_name => $result
            ),*
        }
    };
}

pub use {example_tests, known_input_tests};
