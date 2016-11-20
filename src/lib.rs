use std::rc::{Rc, Weak};


pub struct WeakVec<T: ?Sized>(Vec<Weak<T>>);


impl<T: ?Sized> WeakVec<T> {
    #[inline]
    pub fn new(v: Vec<Weak<T>>) -> WeakVec<T> {
        WeakVec(v)
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut Vec<Weak<T>> {
        &mut self.0
    }

    #[inline]
    pub fn get(&self) -> &Vec<Weak<T>> {
        &self.0
    }

    #[inline]
    pub fn gc(&mut self) {
        // retain only weak references that still point somewhere
        self.0.retain(|weak| weak.upgrade().is_some())
    }

    #[inline]
    pub fn push(&mut self, weak: Weak<T>) {
        // FIXME: optimization possible: use first empty slot

        self.get_mut().push(weak);
        self.gc();
    }
}

pub struct Multiplexer<T> {
    listeners: WeakVec<Observer<Item = T>>,
}

impl<T> Multiplexer<T> {
    pub fn new() -> Multiplexer<T> {
        Multiplexer { listeners: WeakVec::new(Vec::new()) }
    }

    #[inline]
    pub fn register(&mut self, listener: Rc<Observer<Item = T>>) {
        self.listeners.push(Rc::downgrade(&listener))
    }

    pub fn distribute(&self, mut item: T) -> Option<T> {
        for lref in self.listeners.get().iter().filter_map(|l| l.upgrade()) {
            // pass item to every listener until one consumes it
            item = if let Some(nitem) = lref.handle(item) {
                nitem
            } else {
                return None;
            };
        }

        Some(item)
    }
}

pub trait Observer {
    type Item;

    fn handle(&self, item: Self::Item) -> Option<Self::Item>;
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::*;

    struct Num(pub u32);

    struct NumWaiter {
        pub n: u32,
        pub found: bool,
    }

    impl NumWaiter {
        fn new(n: u32) -> NumWaiter {
            NumWaiter {
                n: n,
                found: false,
            }
        }
    }

    impl Observer for NumWaiter {
        type Item = Num;

        fn handle(&self, item: Num) -> Option<Num> {
            if self.n != item.0 {
                Some(item)
            } else {
                // consume if number matches
                None
            }
        }
    }

    #[test]
    fn basic_observer() {
        let mut mp = Multiplexer::new();

        let w_1 = Rc::new(NumWaiter::new(1));
        let w_7 = Rc::new(NumWaiter::new(7));
        let w_99 = Rc::new(NumWaiter::new(9));

        mp.register(w_1.clone());
        mp.register(w_7.clone());
        mp.register(w_99.clone());
    }
}
