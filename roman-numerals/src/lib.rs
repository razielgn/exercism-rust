use std::fmt;

pub struct Roman(u32);

impl From<u32> for Roman {
    fn from(n: u32) -> Self { Roman(n) }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut dec = self.0;

        for &(m, s) in TABLE {
            while dec >= m {
                try!(write!(f, "{}", s));
                dec -= m;
            }
        }

        Ok(())
    }
}

const TABLE: &'static [(u32, &'static str)] = &[(1000, "M"),
                                                (900, "CM"),
                                                (500, "D"),
                                                (400, "CD"),
                                                (100, "C"),
                                                (90, "XC"),
                                                (50, "L"),
                                                (40, "XL"),
                                                (10, "X"),
                                                (9, "IX"),
                                                (5, "V"),
                                                (4, "IV"),
                                                (1, "I")];
