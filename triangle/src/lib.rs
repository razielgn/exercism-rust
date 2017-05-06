extern crate num_traits;

use num_traits::Num;

pub struct Triangle<N> {
    sides: [N; 3],
}

impl<N: Num + PartialOrd + Copy> Triangle<N> {
    pub fn build(mut s: [N; 3]) -> Result<Self, ()> {
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let not_all_zeros = s.iter().all(|l| *l > N::zero());
        let possible = s[2] <= s[0] + s[1];

        if not_all_zeros && possible {
            Ok(Triangle { sides: s })
        } else {
            Err(())
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.windows(2).all(|w| w[0] == w[1])
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.windows(2).filter(|w| w[0] == w[1]).count() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.windows(2).all(|w| w[0] != w[1])
    }
}
