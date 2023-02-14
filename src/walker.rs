use crate::coordinates::Dir;

pub trait Walkable {
    type W: Walker;
    fn walker(&self, dir: Dir) -> Self::W;
}

pub trait Walker {
    type Item;

    fn walk_one(&self) -> Option<Self::Item> {
        self.walk(1)
    }

    fn walk(&self, length: usize) -> Option<Self::Item>;
}
