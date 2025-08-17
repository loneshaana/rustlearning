use std::sync::{Arc, Mutex};

pub struct Sender<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

pub struct Receiver<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

struct Inner<T> {
    queue: Vec<T>, // why not have it a linkedlist.
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner { queue: Vec::new() };

    let inner = Arc::new(Mutex::new(inner));
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}
