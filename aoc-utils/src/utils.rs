/// Iterate over all unique pairs of elements in a slice
pub struct PairsIterator<'a, T> {
    slice: &'a [T],
    index1: usize,
    index2: usize,
}

impl<'a, T> PairsIterator<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        Self {
            slice,
            index1: 0,
            index2: 1,
        }
    }
}

impl<'a, T> Iterator for PairsIterator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index1 < self.slice.len() {
            if self.index2 < self.slice.len() {
                let pair = (&self.slice[self.index1], &self.slice[self.index2]);
                self.index2 += 1;
                Some(pair)
            } else {
                self.index1 += 1;
                self.index2 = self.index1 + 1;
                self.next()
            }
        } else {
            None
        }
    }
}

pub trait SliceUtils<T> {
    fn pairs(&self) -> PairsIterator<'_, T>;
}

impl<T> SliceUtils<T> for [T] {
    fn pairs(&self) -> PairsIterator<'_, T> {
        PairsIterator::new(self)
    }
}

/// Extensions to [[u8]] for ASCII-specific operations
pub trait AsciiUtils<'a> {
    type Lines: Iterator<Item = &'a [u8]>;
    /// Iterate over the lines in a slice of ASCII bytes
    fn ascii_lines(&self) -> Self::Lines;

    /// Parses this byte slice into another type as an ASCII string.
    ///
    /// This is equivalent to `str::parse` but for ASCII bytes.
    ///
    /// # Errors
    ///
    /// Will return `Err` if it’s not possible to parse this byte slice into the
    /// desired type.
    fn parse<'f, F>(self) -> Result<F, F::Error>
    where
        F: FromAscii<Slice<'f> = Self>,
        Self: Sized,
    {
        F::from_ascii(self)
    }

    /// Interpret the slice as a grid of cells that can be converted from ASCII
    /// characters, where each line is the same length.
    ///
    /// # Errors
    ///
    /// Will return `Err` if it’s not possible to parse every byte into the
    /// desired cell type.
    fn grid_like<Cell: TryFrom<u8>>(&self) -> Result<GridLike<Cell>, Cell::Error> {
        // TODO: probably not optimized
        let cells = self
            .ascii_lines()
            .flat_map(|line| line.iter().map(|&c| c.try_into()))
            .collect::<Result<Vec<Cell>, Cell::Error>>()?;
        let width = self.ascii_lines().next().map_or(0, <[u8]>::len);
        let height = self.ascii_lines().count();
        Ok(GridLike {
            cells,
            width,
            height,
        })
    }
}

impl<'a> AsciiUtils<'a> for &'a [u8] {
    type Lines = LinesIterator<'a>;
    fn ascii_lines(&self) -> LinesIterator<'a> {
        LinesIterator::new(self)
    }
}

/// Iterate over the lines in a slice of ASCII bytes
pub struct LinesIterator<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> LinesIterator<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice, index: 0 }
    }
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slice.len() {
            let start = self.index;
            let slice = &self.slice[start..];
            let end = if let Some(newline) = slice.iter().position(|&c| c == b'\n') {
                self.index += newline + 1;
                start + newline
            } else {
                self.index = self.slice.len();
                self.slice.len()
            };
            Some(&self.slice[start..end])
        } else {
            None
        }
    }
}

/// Similar to `FromStr`, but for ASCII bytes
pub trait FromAscii: Sized {
    type Slice<'a>;
    type Error;

    /// Parses this byte slice into another type as an ASCII string.
    ///
    /// This is equivalent to `FromStr::from_str` but for ASCII bytes.
    ///
    /// # Errors
    ///
    /// Will return `Err` if it’s not possible to parse this byte slice into the
    /// desired type.
    fn from_ascii(s: Self::Slice<'_>) -> Result<Self, Self::Error>;
}

macro_rules! impl_for_ascii_for_number_type {
    ($($x:ty),+) => {
        $(
            impl FromAscii for $x {
                type Slice<'a> = &'a [u8];
                type Error = std::num::ParseIntError;
                fn from_ascii(s: Self::Slice<'_>) -> Result<Self, Self::Error> {
                    std::str::from_utf8(s).unwrap().parse()
                }
            }
        )+
    };
}

impl_for_ascii_for_number_type!(u8, i8, u16, i16, u32, i32, u64, i64);

/// A grid of cells that can be converted from ASCII characters.
///
/// This is a helper struct for implementing [`FromGridLike`] for a type. It does
/// not directly implement any grid utility methods, because they might be
/// problem-specific and are left to the implementer of [`FromGridLike`].
pub struct GridLike<Cell> {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl<Cell> GridLike<Cell> {
    #[must_use]
    pub fn into_grid<G>(self) -> G
    where
        G: FromGridLike<Cell = Cell>,
        Cell: TryFrom<u8>,
    {
        let GridLike {
            cells,
            width,
            height,
        } = self;
        G::from_cells(cells, width, height)
    }
}

pub trait FromGridLike
where
    Self: Sized,
{
    type Cell: TryFrom<u8>;
    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self;
}

pub struct InvalidCharacter(pub u8);

impl core::fmt::Debug for InvalidCharacter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid character: {} ({})", self.0 as char, self.0)
    }
}

#[macro_export]
macro_rules! grid_cell_enum {
    (
        $(#[$attrs:meta])*
        enum $name:ident {
            $($variant:ident => $value:expr),*$(,)?
        }
    )
        => {
            $(#[$attrs])*
            enum $name {
                $($variant,)*
            }

            impl TryFrom<u8> for $name {
                type Error = $crate::utils::InvalidCharacter;
                fn try_from(c: u8) -> Result<Self, $crate::utils::InvalidCharacter> {
                    match c {
                        $($value => Ok(Self::$variant),)*
                        c => Err($crate::utils::InvalidCharacter(c)),
                    }
                }
            }

            impl core::fmt::Display for $name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        $($name::$variant => write!(f, "{}", $value as char),)*
                    }
                }
            }
        }
}

pub use grid_cell_enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parity {
    Even = 0,
    Odd = 1,
}

impl std::ops::Not for Parity {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
        }
    }
}

pub trait NumberExt: Sized {
    #[must_use]
    fn greatest_common_divisor(self, other: Self) -> Self;
    #[must_use]
    fn least_common_multiple(self, other: Self) -> Self;
    #[must_use]
    fn parity(self) -> Parity;
    #[must_use]
    fn split_odd_even(self) -> (Self, Self);

    #[must_use]
    fn zero() -> Self;
    #[must_use]
    fn one() -> Self;
}

impl<T> NumberExt for T
where
    T: core::ops::Rem<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::BitAnd<Output = Self>
        + Copy
        + PartialOrd
        + From<bool>,
{
    // this is a bit of a hack: From<bool> is implemented for all primitive
    // integers and provides 0 and 1 (sadly not const)
    fn zero() -> Self {
        false.into()
    }
    fn one() -> Self {
        true.into()
    }

    fn parity(self) -> Parity {
        if self & Self::one() == Self::zero() {
            Parity::Even
        } else {
            Parity::Odd
        }
    }

    fn split_odd_even(self) -> (Self, Self) {
        let two = Self::one() + Self::one();
        let even = self / two;
        let odd = even + (self % two);
        (odd, even)
    }

    fn greatest_common_divisor(self, other: Self) -> Self {
        let mut a = self;
        let mut b = other;
        while b != Self::zero() {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn least_common_multiple(self, other: Self) -> Self {
        self * other / self.greatest_common_divisor(other)
    }
}

pub trait NumberIteratorExt: Sized {
    fn least_common_multiple(self) -> Self::Item
    where
        Self: Iterator,
        Self::Item: NumberExt,
    {
        self.fold(Self::Item::one(), Self::Item::least_common_multiple)
    }
}

impl<T> NumberIteratorExt for T where T: Iterator {}

pub trait NumberDigitsExt: Copy {
    type MaxDigits;
    /// Write the decimal digits of the number into the provided slice, starting
    /// from the least significant digit.
    ///
    /// # Errors
    ///
    /// Returns `Err(BufferTooSmall)` if the number of digits exceeds the size
    /// of the buffer.
    fn digits_in(self, slice: &mut [u8]) -> Result<usize, BufferTooSmall>;
    /// Returns the decimal digits of the number as a vector, starting from the
    /// least significant digit.
    fn digits(self) -> Vec<u8>;
}

pub struct MaxDigits<T>(std::marker::PhantomData<T>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BufferTooSmall;

impl core::fmt::Debug for BufferTooSmall {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "digit count exceeds the size of the buffer")
    }
}

macro_rules! impl_number_digits_ext_for_num_type {
    ($($x:ty),+) => {
        $(
            impl MaxDigits<$x> {
                /// The maximum number of decimal digits that can be represented by this type.
                pub const COUNT: usize = const { (<$x>::MAX).ilog10() as usize + 1 };
                /// Returns an array of zeros with the maximum number of decimal
                /// digits that can be represented by this type.
                pub const fn array() -> [u8; Self::COUNT] {
                    [0u8; Self::COUNT]
                }
            }

            impl NumberDigitsExt for $x {
                type MaxDigits = MaxDigits<Self>;

                fn digits_in(self, slice: &mut [u8]) -> Result<usize, BufferTooSmall> {
                    let mut num = self;
                    let mut index = 0;
                    if num == 0 {
                        slice[index] = 0;
                        index += 1;
                    }
                    while num > 0 {
                        if index == slice.len() {
                            debug_assert!(index < Self::MaxDigits::COUNT);
                            return Err(BufferTooSmall);
                        }
                        #[allow(clippy::cast_possible_truncation)]
                        {
                            slice[index] = (num % 10) as u8;
                        }
                        num /= 10;
                        index += 1;
                    }
                    Ok(index)
                }

                fn digits(self) -> Vec<u8> {
                    let mut digits = Self::MaxDigits::array().to_vec();
                    let size = self.digits_in(&mut digits).unwrap();
                    digits.truncate(size);
                    digits
                }
            }
        )+
    };
}

// implemented only for signed types, because I don't want to deal with the minus sign
impl_number_digits_ext_for_num_type!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_iterator() {
        let mut iter = PairsIterator::new(&[1, 2, 3, 4]);
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&1, &3)));
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), Some((&2, &4)));
        assert_eq!(iter.next(), Some((&3, &4)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pairs_iterator_too_small() {
        let mut iter = PairsIterator::new(&[1]);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pairs_iterator_empty() {
        let mut iter = PairsIterator::new(&[1]);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ascii_lines() {
        let mut iter = LinesIterator::new(b"abc\ndef\nghi\n");
        assert_eq!(iter.next(), Some(&b"abc"[..]));
        assert_eq!(iter.next(), Some(&b"def"[..]));
        assert_eq!(iter.next(), Some(&b"ghi"[..]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ascii_lines_unterminated() {
        let mut iter = LinesIterator::new(b"abc\ndef\nghi");
        assert_eq!(iter.next(), Some(&b"abc"[..]));
        assert_eq!(iter.next(), Some(&b"def"[..]));
        assert_eq!(iter.next(), Some(&b"ghi"[..]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ascii_lines_single_line() {
        let mut iter = LinesIterator::new(b"abc");
        assert_eq!(iter.next(), Some(&b"abc"[..]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ascii_lines_empty() {
        let mut iter = LinesIterator::new(b"");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ascii_lines_empty_lines() {
        let mut iter = LinesIterator::new(b"abc\n\nghi");
        assert_eq!(iter.next(), Some(&b"abc"[..]));
        assert_eq!(iter.next(), Some(&b""[..]));
        assert_eq!(iter.next(), Some(&b"ghi"[..]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ascii_parse() {
        struct Foo;
        impl FromAscii for Foo {
            type Slice<'a> = &'a [u8];
            type Error = ();
            fn from_ascii(s: &[u8]) -> Result<Self, Self::Error> {
                assert_eq!(s, b"abc");
                Ok(Foo)
            }
        }
        assert!(matches!(b"abc".parse::<Foo>(), Ok(Foo)));
        let foo = vec![b'a', b'b', b'c'];
        assert!(matches!(foo.as_slice().parse::<Foo>(), Ok(Foo)));
    }

    #[test]
    fn ascii_grid() {
        let grid = b"abc\ndef\nghi\njkl".as_slice().grid_like::<u8>().unwrap();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.cells, b"abcdefghijkl".to_vec(),);
    }

    #[test]
    fn max_digits() {
        let x = u64::MAX;
        let decimal = format!("{x}");
        let digits = x.digits();
        assert_eq!(digits.len(), decimal.len());
        assert_eq!(digits.len(), <u64 as NumberDigitsExt>::MaxDigits::COUNT);
    }

    #[test]
    fn digits() {
        assert_eq!(0u16.digits(), vec![0]);
        assert_eq!(1u16.digits(), vec![1]);
        assert_eq!(10u16.digits(), vec![0, 1]);
        assert_eq!(123u16.digits(), vec![3, 2, 1]);
        assert_eq!(12345u16.digits(), vec![5, 4, 3, 2, 1]);
        assert_eq!(u16::MAX.digits(), vec![5, 3, 5, 5, 6]);
        assert_eq!(0u32.digits(), vec![0]);
        assert_eq!(u32::MAX.digits(), [5, 9, 2, 7, 6, 9, 4, 9, 2, 4]);
        assert_eq!(0u64.digits(), vec![0]);
        assert_eq!(
            u64::MAX.digits(),
            [5, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1]
        );
    }

    #[test]
    fn digits_in() {
        let mut buf = MaxDigits::<u64>::array();
        let size = u64::MAX.digits_in(&mut buf).unwrap();
        assert_eq!(size, buf.len());
        assert_eq!(
            buf,
            [5, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1]
        );
        let size = 0u64.digits_in(&mut buf).unwrap();
        assert_eq!(size, 1);
        assert_eq!(
            buf,
            [0, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1]
        );
    }

    #[test]
    fn digits_in_small_buf() {
        let mut buf = [0; 2];
        let result = 100u64.digits_in(&mut buf);
        assert_eq!(result, Err(BufferTooSmall));
    }
}

#[derive(Debug)]
pub struct Annotate<T, A> {
    pub value: T,
    pub annotation: A,
}

impl<T, A> PartialEq for Annotate<T, A>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl<T, A> Eq for Annotate<T, A> where T: Eq {}

impl<T, A> PartialOrd for Annotate<T, A>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, A> Ord for Annotate<T, A>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T, A> Clone for Annotate<T, A>
where
    T: Clone,
    A: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            annotation: self.annotation.clone(),
        }
    }
}

pub trait AnnotateExt<T, A> {
    fn annotate(self, annotation: A) -> Annotate<T, A>;
}

impl<T, A> AnnotateExt<T, A> for T {
    fn annotate(self, annotation: A) -> Annotate<T, A> {
        Annotate {
            value: self,
            annotation,
        }
    }
}
