use crate::Dir;
use std::fmt::Debug;

#[derive(Debug)]
pub struct WalkerIterator<W: Walkable + Debug> {
    walkable: W,
    dir: Dir,
    ix: usize,
}

impl<W: Walkable + Debug> WalkerIterator<W> {
    pub fn new(walkable: W, dir: Dir) -> Self {
        Self {
            walkable,
            dir,
            ix: 1,
        }
    }
}

impl<W: Walkable<WItem = W> + Debug> Iterator for WalkerIterator<W> {
    type Item = W;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.walkable.walker(self.dir).walk(self.ix);
        self.ix += 1;
        next
    }
}

pub trait Walkable {
    type WItem: Walkable;
    type W: Walker<WItem = Self::WItem>;

    fn walker(&self, dir: Dir) -> Self::W;
}

pub trait Walker: IntoIterator {
    type WItem: Walkable;

    fn walk_one(&self) -> Option<Self::WItem> {
        self.walk(1)
    }

    fn walk(&self, length: usize) -> Option<Self::WItem>;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Dummy(usize);

    impl Walkable for Dummy {
        type WItem = Dummy;
        type W = DummyWalker;

        fn walker(&self, dir: Dir) -> Self::W {
            DummyWalker(*self, dir)
        }
    }

    struct DummyWalker(Dummy, Dir);

    impl Walker for DummyWalker {
        type WItem = Dummy;

        fn walk(&self, length: usize) -> Option<Self::WItem> {
            return if length > 9 {
                None
            } else {
                Some(Dummy(self.0 .0 + length))
            };
        }
    }

    impl IntoIterator for DummyWalker {
        type Item = Dummy;

        type IntoIter = WalkerIterator<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            WalkerIterator::new(self.0, Dir::Up)
        }
    }

    #[test]
    fn test_walker_walk_with_length() {
        assert_eq!(DummyWalker(Dummy(0), Dir::Down).walk(5), Some(Dummy(5)));
        assert_eq!(DummyWalker(Dummy(1), Dir::Down).walk(5), Some(Dummy(6)));
        assert_eq!(DummyWalker(Dummy(1), Dir::Down).walk(10), None);
    }

    #[test]
    fn test_walk_iterator() {
        let mut i = WalkerIterator::new(Dummy(1), Dir::Down);
        assert_eq!(i.next().unwrap(), Dummy(2));
        assert_eq!(i.next().unwrap(), Dummy(3));
        assert_eq!(i.next().unwrap(), Dummy(4));
        assert_eq!(i.next().unwrap(), Dummy(5));
        assert_eq!(i.next().unwrap(), Dummy(6));
        assert_eq!(i.next().unwrap(), Dummy(7));
        assert_eq!(i.next().unwrap(), Dummy(8));
        assert_eq!(i.next().unwrap(), Dummy(9));
        assert_eq!(i.next().unwrap(), Dummy(10));
        assert_eq!(i.next(), None);
    }
}
