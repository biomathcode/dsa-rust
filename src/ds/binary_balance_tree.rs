// binary tree will have a right, left, data

use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => {
                return Box::new(self);
            }
        };

        // move left to right nod eto right of the start node

        self.right = BinTree(res.left.0.take());

        self.right.set_height();

        res.left = BinTree(Some(Box::new(self)));

        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                } else {
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(
                    Box::new(BinData { data, h: 0, left: BinTree::new(), right: BinTree::new() })
                );
            }
        }
        self.set_height();
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(dp + 1);
            let mut spc = String::new();

            for _ in 0..dp {
                spc.push('.');
            }
            println!("heigth{}: {}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let mut t = BinTree::new();

        t.add_sorted(4);
        t.add_sorted(5);
        t.add_sorted(6);

        t.add_sorted(1);
        t.add_sorted(3);
        t.add_sorted(10);

        t.print_lfirst(0);

        println!("------------------");

        t.rot_left();

        t.print_lfirst(0);
    }
}
