use core::ops::{Range, RangeInclusive};

use super::Transpose;

macro_rules! impl_transpose {
    ($($t:ident: $a:ident;$b:ident),*) => {

impl<$($t),*> Transpose for Range<($($t),*)> {
    type Output = ($(Range<$t>),*);

    fn transpose(self) -> Self::Output {
        let Range { start: ($($a),*), end: ($($b),*) } = self;
        ($($a..$b),*)
    }
}

impl<$($t),*> Transpose for RangeInclusive<($($t),*)> {
    type Output = ($(RangeInclusive<$t>),*);

    fn transpose(self) -> Self::Output {
        let (($($a),*), ($($b),*)) = self.into_inner();
        ($($a..=$b),*)
    }
}

    };
}

impl_transpose!(T0: a0;b0, T1: a1;b1);
impl_transpose!(T0: a0;b0, T1: a1;b1, T2: a2;b2);
impl_transpose!(T0: a0;b0, T1: a1;b1, T2: a2;b2, T3: a3;b3);
