#[cfg_attr(not(feature = "num-traits"), no_std)]

#[cfg(feature = "num-traits")]
extern crate num;

#[cfg(test)]
mod test;

/// An Iterator that returns numbers from the Kolakoski sequence
///
/// Information about the Kolakoski sequence and the first 108 numbers
/// can be found at https://oeis.org/A000002. The source of the numbers
/// used for testing in `test.rs` is also https://oeis.org/A000002.
///
/// This sequence is self-refrential, and the implementation is recursive.
/// Attempts to get a large number of items from the sequence may overflow
/// the stack, depending on the machine. Use with caution!
pub struct Kolakoski
{
    run: usize,
    run_length: u8,
    is_one: bool,
}

#[cfg(feature = "num-traits")]
pub struct KolakoskiNum<N>
{
    run: usize,
    run_length: N,
    is_one: bool,
}

impl Kolakoski
{
    /// Creates a new Kolakoski iterator
    pub fn new() -> Kolakoski
    {
        Kolakoski
        {
            run: 0,
            run_length: 1,
            is_one: true,
        }
    }
}

#[cfg(feature = "num-traits")]
impl<N: Num> KolakoskiNum<N>
{
    /// Creates a new Kolakoski iterator
    pub fn new() -> KolakoskiNum<N>
    {
        KolakoskiNum
        {
            run: 0,
            run_length: N::one(),
            is_one: true,
        }
    }
}

impl Iterator for Kolakoski
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.run_length == 0
        {
            self.is_one = !self.is_one;
            self.run += 1;

            if self.run == 1
            {
                self.run_length = 2;
            }
            else
            {
                self.run_length = Kolakoski::new().nth(self.run).unwrap();
            }
        }

        self.run_length -= 1;

        if self.is_one
        {
            Some(1)
        }
        else
        {
            Some(2)
        }
    }
}

#[cfg(feature = "num-traits")]
use num::traits::Num;

#[cfg(feature = "num-traits")]
impl<N: Num + Clone> Iterator for KolakoskiNum<N>
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.run_length == N::zero()
        {
            self.is_one = !self.is_one;
            self.run += 1;

            if self.run == 1
            {
                self.run_length = N::one() + N::one()
            }
            else
            {
                self.run_length = KolakoskiNum::<N>::new().nth(self.run).unwrap();
            }
        }

        self.run_length = self.run_length.clone() - N::one();

        if self.is_one
        {
            Some(N::one())
        }
        else
        {
            Some(N::one() + N::one())
        }
    }
}