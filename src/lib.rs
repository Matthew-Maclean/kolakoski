#![no_std]

#[cfg(test)]
mod test;

pub struct Kolakoski
{
    run: usize,
    run_length: u8,
    is_one: bool,
}

impl Kolakoski
{
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
                let run = self.run;
                let is_one = self.is_one;

                self.run = 0;
                self.is_one = true;
                self.run_length = 1;

                let run_length = self.nth(run).unwrap();

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