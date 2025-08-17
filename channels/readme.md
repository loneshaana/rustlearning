# channels: a minimal MPSC channel built from Mutex + Condvar

This crate implements a simple multi-producer, single-consumer channel using `Mutex`, `Condvar`, and `VecDeque`.

- API: `channel<T>() -> (Sender<T>, Receiver<T>)`
- FIFO delivery
- Blocking `recv` with spurious-wakeup-safe loop
- Graceful close: `recv` returns `None` once all senders are dropped and the queue is empty
- Multiple producers via `Sender` cloning; single consumer

See implementation in [channels/src/lib.rs](src/lib.rs).

## API overview

- [`Sender<T>`](src/lib.rs)
  - `fn send(&mut self, t: T)` — Enqueue a value and wake one waiter.
  - `impl Clone` — Cloning a `Sender` increments the sender count.
  - `impl Drop` — Dropping a `Sender` decrements the count; if it was the last, wakes a waiting receiver.

- [`Receiver<T>`](src/lib.rs)
  - `fn recv(&mut self) -> Option<T>` — Blocks until an item is available, or returns `None` if the channel is closed (all senders dropped and queue empty).

- [`channel<T>()`](src/lib.rs) — Constructs a `(Sender<T>, Receiver<T>)` pair.

## Behavior and guarantees

- Blocking and wakeups
  - `recv` holds a `Mutex` guard and pops from the `VecDeque`.
  - If empty and there are still senders, it waits on a `Condvar`, releasing the guard while sleeping.
  - On wake, it re-acquires the guard and re-checks (loop handles spurious wakeups).

- Close semantics
  - Internally tracks an active sender count.
  - `recv` returns `None` when the queue is empty and the sender count is zero.

- FIFO ordering
  - Uses `VecDeque` for O(1) push-back/pop-front.

- Thread safety
  - No `unsafe` code; synchronization via `Mutex` + `Condvar`.

## Design notes

Types in [src/lib.rs](src/lib.rs):

- [`struct Shared<T>`](src/lib.rs): holds the `Mutex<Inner<T>>` and `Condvar`.
- [`struct Inner<T>`](src/lib.rs): `queue: VecDeque<T>`, `senders: usize`.

Why `Condvar` is alongside (not inside) the `Mutex`:
- The condition variable coordinates sleeping/waking while the mutex protects the queue and sender count. The waiter releases the guard when sleeping; the waker doesn’t have to contend with a guard held by the sleeper.

Clone/Drop semantics on [`Sender<T>`](src/lib.rs):
- `Clone` locks the mutex and increments `senders`.
- `Drop` decrements; if it hits zero, it notifies one waiter so a pending `recv` can observe closure and return `None`.

## Usage

Basic ping-pong:

```rust
use channels::channel;

fn main() {
    let (mut tx, mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.recv(), Some(42));
}
```

Multiple producers:

```rust
use channels::channel;
use std::thread;

fn main() {
    let (mut tx, mut rx) = channel();
    let mut tx2 = tx.clone();

    let t1 = thread::spawn(move || { tx.send(1); });
    let t2 = thread::spawn(move || { tx2.send(2); });

    t1.join().unwrap();
    t2.join().unwrap();

    let a = rx.recv().unwrap();
    let b = rx.recv().unwrap();
    assert!([1,2].contains(&a) && [1,2].contains(&b) && a != b);

    // When all senders are dropped and queue is empty:
    assert_eq!(rx.recv(), None);
}
```

Note: `send(&mut self, ...)` requires a unique `Sender` reference; clone the sender for each producer thread.

## Included tests

See tests in [src/lib.rs](src/lib.rs):

- `ping_pong` — Sends a value and receives it.
- `closed` — Drops the sender and verifies `recv()` returns `None`.

## How to run

From the crate directory:

```powershell
cd channels
cargo test
```

## Limitations and potential extensions

- Unbounded channel (no capacity/backpressure).
- Single consumer; `Receiver` isn’t clonable.
- `send` requires `&mut self`; share by cloning per thread.
- No fairness guarantees (uses `notify_one`).
- Extensions: bounded capacity (with separate `not_full` condvar), multi-consumer, non-blocking `try_recv`, `recv_timeout`,