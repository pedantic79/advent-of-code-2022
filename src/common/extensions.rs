pub trait MinMaxIterator: Iterator {
    fn min_max<'a, T>(mut self) -> Option<(&'a T, &'a T)>
    where
        T: Ord,
        Self: Iterator<Item = &'a T> + Sized,
    {
        self.next()
            .map(|x| self.fold((x, x), |(min, max), num| (min.min(num), max.max(num))))
    }
}

impl<T: ?Sized> MinMaxIterator for T where T: Iterator {}

pub trait AddIsize
where
    Self: Sized + PartialOrd,
{
    fn checked_add_isize(self, rhs: isize) -> Option<Self>;
    fn checked_add_isize_clamp(self, rhs: isize, max: Self) -> Option<Self> {
        self.checked_add_isize(rhs).filter(|x| x < &max)
    }
}

impl AddIsize for usize {
    fn checked_add_isize(self, rhs: isize) -> Option<Self> {
        let amount = Self::try_from(rhs.abs()).ok()?;
        if rhs < 0 {
            self.checked_sub(amount)
        } else {
            self.checked_add(amount)
        }
    }
}
