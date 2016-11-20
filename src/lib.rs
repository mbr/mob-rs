use std::marker::PhantomData;

use std::rc::Weak;

pub struct Multiplexer<T> {
    listeners: Vec<Weak<Observer<Item = T>>>,
    _item: PhantomData<T>,
}

impl<T> Multiplexer<T> {
    pub fn new() -> Multiplexer<T> {
        Multiplexer {
            listeners: Vec::new(),
            _item: PhantomData,
        }
    }

    pub fn register(&mut self, listener: Weak<Observer<Item = T>>) {
        // FIXME: gc. could put this in an extra structure
        // FIXME: optimization possible: use first empty slot
        self.listeners.push(listener);
    }

    pub fn distribute(&self, mut item: T) -> Option<T> {
        for lref in self.listeners.iter().filter_map(|l| l.upgrade()) {
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
