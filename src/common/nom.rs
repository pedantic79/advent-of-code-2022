use std::ops::RangeFrom;

use nom::{
    character::complete, combinator::map, error::ParseError, AsChar, Compare, IResult, InputIter,
    InputLength, InputTake, Slice,
};

pub fn nom_usize<T>(s: T) -> IResult<T, usize>
where
    T: InputIter + Slice<RangeFrom<usize>> + InputLength,
    <T as InputIter>::Item: AsChar,
{
    map(nom::character::complete::u64, |x| x as usize)(s)
}

pub fn nom_isize<T>(s: T) -> IResult<T, isize>
where
    T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
    <T as InputIter>::Item: AsChar,
    T: for<'a> Compare<&'a [u8]>,
{
    map(nom::character::complete::i64, |x| x as isize)(s)
}

macro_rules! uints {
    ($($n:ident, $t:tt)+) => {
        $(
        pub fn $n<T>(s: T) -> IResult<T, $t>
        where
            T: InputIter + Slice<RangeFrom<usize>> + InputLength,
            <T as InputIter>::Item: AsChar,
        {
            complete::$t(s)
        }
        )+
    };
}

uints! { nom_u8,u8 nom_u16,u16 nom_u32,u32 nom_u64,u64 nom_u128,u128}

macro_rules! ints {
    ($($n:ident, $t:tt)+) => {
        $(
            pub fn $n<T, E: ParseError<T>>(s: T) -> IResult<T, $t, E>
                where
                T: InputIter + Slice<RangeFrom<usize>> + InputLength + InputTake + Clone,
                <T as InputIter>::Item: AsChar,
                T: for <'a> Compare<&'a[u8]>,
            {
                complete::$t(s)
            }
        )+
    };
}

ints! { nom_i8,i8 nom_i16,i16 nom_i32,i32 nom_i64,i64 nom_i128,i128}
