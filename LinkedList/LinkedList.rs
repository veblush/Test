#[crate_id = "LinkedList#0.1"];

#[deriving(Clone)]
pub enum Node<T> {
    Cons(T, ~Node<T>),
    Nil
}

pub struct List<T> {
    Head: Node<T>,
    Count: int
}

impl<T: Clone> List<T> {
    pub fn new() -> List<T> {
        List { Head: Nil, Count: 0 }
    }

    pub fn push(self, value: T) -> List<T> {
        List { Head: Cons(value, ~self.Head.clone()), Count: self.Count + 1 }
    }

    pub fn pop(self) -> List<T> {
        let count = self.Count;
        match self.Head {
           Cons(_, ~node) => List { Head: node.clone(), Count: count - 1 },
           Nil => { self }
        }
    }

    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator { Cur: &self.Head }
    }
}

struct ListIterator<'a, T> {
    priv Cur: &'a Node<T>
}

impl<'a, T: Clone> Iterator<&'a T> for ListIterator<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        match *(self.Cur) {
            Cons(ref x, ~ref y) => { self.Cur = y; Some(x) }
            Nil => { None }
        }
    }
}

pub fn print<T: ::std::fmt::Default + Clone>(list: &List<T>) {
    for i in list.iter() {
        println!("{}", *i);
    }
}
