use std::str::FromStr;

pub fn parse_pair<T: FromStr>(s: &str) -> Option<(T, T)> {
    parse_split_once(s, ',')
}

pub fn parse_range<T: FromStr>(s: &str) -> Option<(T, T)> {
    parse_split_once(s, "..")
}

pub fn parse_split_once<'a, T, P>(s: &'a str, p: P) -> Option<(T, T)>
where
    T: FromStr,
    P: stable_pattern::Pattern<'a>,
{
    let (l, r) = pattern::split_once(s, p)?;

    let l = l.parse().ok()?;
    let r = r.parse().ok()?;
    Some((l, r))
}

mod pattern {
    use stable_pattern::{Pattern, Searcher};

    pub fn split_once<'a, P: Pattern<'a>>(s: &'a str, delimiter: P) -> Option<(&'a str, &'a str)> {
        let (start, end) = delimiter.into_searcher(s).next_match()?;
        // SAFETY: `Searcher` is known to return valid indices.
        unsafe { Some((s.get_unchecked(..start), s.get_unchecked(end..))) }
    }
}
