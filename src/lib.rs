use std::rc::Weak;


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

    pub fn register(&mut self, listener: Weak<Observer<Item = T>>) {
        self.listeners.push(listener);
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
    #[test]
    fn it_works() {}
}
