pub enum Side<I> {
    Left(I),
    Right(I),
}

pub struct Comparer<I: Ord, Left: Iterator<Item = I>, Right: Iterator<Item = I>> {
    left: Left,
    right: Right,
    prev_left: Option<I>,
    prev_right: Option<I>,
}

impl<I: Ord, Left: Iterator<Item = I>, Right: Iterator<Item = I>> Comparer<I, Left, Right> {
    fn new(left: Left, right: Right) -> Comparer<I, Left, Right> {
        Comparer {
            left,
            right,
            prev_left: None,
            prev_right: None,
        }
    }

    fn init(&mut self) {
        self.prev_left = self.left.next();
        self.prev_right = self.right.next();
    }
}

impl<I: Ord, Left: Iterator<Item = I>, Right: Iterator<Item = I>> Iterator
    for Comparer<I, Left, Right>
{
    type Item = Side<I>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.prev_left.take(), self.prev_right.take()) {
                (None, None) => return None,
                (Some(left), Some(right)) if left == right => {
                    self.prev_left = self.left.next();
                    self.prev_right = self.right.next();
                    continue;
                }
                (Some(left), Some(right)) if left > right => {
                    self.prev_left = Some(left);
                    self.prev_right = self.right.next();
                    return Some(Side::Right(right));
                }
                (Some(left), Some(right)) if left < right => {
                    self.prev_left = self.left.next();
                    self.prev_right = Some(right);
                    return Some(Side::Left(left));
                }
                (None, Some(right)) => {
                    self.prev_right = self.right.next();
                    return Some(Side::Right(right));
                }
                (Some(left), None) => {
                    self.prev_left = self.left.next();
                    return Some(Side::Left(left));
                }
                (_, _) => return None,
            };
        }
    }
}

pub fn compare<I: Ord, Left: Iterator<Item = I>, Right: Iterator<Item = I>>(
    left: Left,
    right: Right,
) -> Comparer<I, Left, Right> {
    let mut c = Comparer::new(left, right);
    c.init();
    c
}
