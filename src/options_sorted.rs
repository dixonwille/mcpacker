pub trait Sort<T: Ord> {
    fn sort_unstable(&mut self);
    fn len(&self) -> usize;
    fn remove(&mut self, index: usize) -> T;
    fn binary_search(&self, element: &T) -> Result<usize, usize>;
    fn insert(&mut self, index: usize, element: T);
    fn append(&mut self, elements: &mut Self);

    fn add(&mut self, element: T) {
        match self.binary_search(&element) {
            Ok(_) => {}
            Err(p) => self.insert(p, element),
        }
    }

    fn add_multiple(&mut self, elements: &mut Self) {
        match elements.len() {
            0 => {}
            1 => {
                self.add(elements.remove(0));
            }
            _ => {
                self.append(elements);
                self.sort_unstable();
            }
        }
    }

    fn remove_element(&mut self, element: T) -> Option<T> {
        match self.binary_search(&element) {
            Ok(p) => Some(self.remove(p)),
            Err(_) => None,
        }
    }
}

impl<T: Ord> Sort<T> for Vec<T> {
    fn sort_unstable(&mut self){
        <[T]>::sort_unstable(self)
    }
    fn len(&self) -> usize{
        <[T]>::len(self)
    }
    fn remove(&mut self, index: usize) -> T{
        Vec::remove(self, index)
    }
    fn binary_search(&self, element: &T) -> Result<usize, usize>{
        <[T]>::binary_search(self, element)
    }
    fn insert(&mut self, index: usize, element: T){
        Vec::insert(self, index, element)
    }
    fn append(&mut self, elements: &mut Self){
        Vec::append(self, elements)
    }
}

pub trait OptionSort<I: Ord, T: Sort<I> + Default> {
    fn add(&mut self, element: I);
    fn add_multiple(&mut self, elements: &mut T);
    fn remove_element(&mut self, element: I) -> Option<I>;
}


impl<I: Ord, T: Sort<I> + Default> OptionSort<I, T> for Option<T> {
    fn add(&mut self, element: I){
        match self{
            Some(elements) => {
                elements.add(element)
            },
            None => {
                let mut elements: T = Default::default();
                elements.add(element)
            }
        }
    }
    fn add_multiple(&mut self, elements: &mut T){
        match self {
            Some(my_elements) => {
                my_elements.add_multiple(elements)
            },
            None => {
                let mut my_elements: T = Default::default();
                my_elements.add_multiple(elements)
            }
        }
    }
    fn remove_element(&mut self, element: I) -> Option<I>{
        match self {
            Some(elements) => {
                elements.remove_element(element)
            },
            None => None
        }
    }
}