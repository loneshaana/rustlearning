use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// A Mutex is boolean semaphore effectively
// Arc is needed to have a shared inner datastructure for both sender and receiver.

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

/*
if we use #[derive(Clone)] annotation it will basically translate to this to the below Code
we notice here the Compiler has made it mandidory that T should be Cloneable, but if we see
our inner object in Sender it is wraped in Arc which implements Clone irrespective if T is cloneable or not
So what we ideally want is a Clone on Sender but enforcing the T to be cloneable for that reason we can't use $[derive(Clone)]

impl<T:Clone> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // ....
    }
}
*/

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            /*
            inner: self.inner.clone(), // This can't be used.
            having clone() on inner is technically legal but rust won't know if the clone method is to call
            the data which is with the Arc or to call the clone method of the Arc because Arc is basically dereferencing
            the inner type, so what we usually want to use is Arc::clone(&self.inner) to say specifically want to clone the Arc
             */
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap(); // What if the thread failed to access the lock.
        queue.push_back(t);
        drop(queue); //drops the lock, when other notify wakes up the other thread it can take the lock immediately.

        // and if any thread is in sleep and is waiting for the data
        // we will use the notify_one method to wake it up.
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        /*
        queue.pop_front().unwrap()
        pop_front returns and option and what if there is no element is the queue.
        what practically we want to do is to wait till there is some data to receive.
        For that we use Condvar in Inner
         */
        loop {
            let mut queue = self.inner.queue.lock().unwrap();
            match queue.pop_front() {
                Some(t) => return t, // releases the mutex
                None => {
                    self.inner.available.wait(queue).unwrap(); // wait requires you give up the guard and then wait, if it wakes up it take the mutex lock for you
                }
            }
        }
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>, // why not have it a linkedlist.
    available: Condvar,
    /*
    the condvar needs to be outside the mutex, imagine you're currently holding the mutex and  u relalize you to
    wake other people up , the person u wake up has to take the mutex, but you are currently holding the mutex and they try to take the mutex
    but instead they go to sleep and it goes into the deadlock.
    */
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };

    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
}
