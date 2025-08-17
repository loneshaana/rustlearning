use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// A Mutex is boolean semaphore effectively
// Arc is needed to have a shared inner datastructure for both sender and receiver.

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
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
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner); // release lock
        Sender {
            /*
            inner: self.inner.clone(), // This can't be used.
            having clone() on inner is technically legal but rust won't know if the clone method is to call
            the data which is with the Arc or to call the clone method of the Arc because Arc is basically dereferencing
            the inner type, so what we usually want to use is Arc::clone(&self.inner) to say specifically want to clone the Arc
             */
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;

        if inner.senders == 0 {
            self.shared.available.notify_one(); // If it was the last one , notifiy if the receiver is waiting so wakes up.
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap(); // What if the thread failed to access the lock.
        inner.queue.push_back(t);
        drop(inner); //drops the lock, when other notify wakes up the other thread it can take the lock immediately.

        // and if any thread is in sleep and is waiting for the data
        // we will use the notify_one method to wake it up.
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        /*
        queue.pop_front().unwrap()
        pop_front returns and option and what if there is no element is the queue.
        what practically we want to do is to wait till there is some data to receive.
        For that we use Condvar in Inner
         */
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                } // releases the mutex
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap(); // wait requires you give up the guard and then wait, if it wakes up it take the mutex lock for you
                }
            }
        }
    }
}

// #[derive(Default)], we cannot add Default here that requires T to be Default.
/*
    we are creating this Inner within shared with the count of total sender because
    If a receiver is waiting for the data to receive and there are no senders left all are dropped then in that case
    the receiver will never wake up and would infinitely wait for the data to receive.

    So here we create an Inner type within the Shared and have a usize of senders to track the number of sender
    and upon Sender drop we wake the receiver if the count of senders got reduced to 0.
*/
struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
    /*
    the condvar needs to be outside the mutex, imagine you're currently holding the mutex and  u relalize you to
    wake other people up , the person u wake up has to take the mutex, but you are currently holding the mutex and they try to take the mutex
    but instead they go to sleep and it goes into the deadlock.
    */
}

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };

    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };

    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
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
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel::<i32>();
        drop(rx);
        tx.send(42);
        // assert_eq!(rx.recv(), None);
    }
}
