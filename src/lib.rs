#![no_std]

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
                // store current status (don't bother with run_length, because we will chage it)
                let run = self.run;
                let is_one = self.is_one;

                // reset the status of the Iterator
                self.run = 0;
                self.is_one = true;
                self.run_length = 1;

                // get the run length for the current run
                // If you want too many items from the sequence, you will overflow the stack!
                let run_length = self.nth(run).unwrap();

                // re-set (not reset!) the status, including the new run_length value
                self.run = run;
                self.run_length = run_length;
                self.is_one = is_one;
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