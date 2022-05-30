#![allow(unused)]
use std::sync::{mpsc, Mutex, Arc, Condvar};
use std::thread;
use std::time::Duration;

/** In this week's lecture, we have looked at using concurrency in Rust.
We have looked at:

 - How to spawn threads with `thread::spawn(move ||{ ... })`.
 - How to use `join` handles to wait until a child thread has finished.
 - How to use channels and `Sender<T>` and `Receiver<T>` to do message passing.
 - How to use `Arc<T>` to share references to data among multiple threads.
 - How to use `Mutex<T>` to safely encapsulate shared mutable state.
 - How to use `Condvar` to wait until a condition holds, without a spin loop.

To complete this exercise, you should extend various functions, and replace all
uses of `unimplemented!()` with working code. This file contains some tests,
but these tests are very much non-exhaustive, so you have to convince yourself
that your solutions are correct.


### Documentation ###

You can learn about these topics from the Rust book, chapter 16:

  https://doc.rust-lang.org/book/ch16-00-concurrency.html

And at the API documentation for the thread and sync modules:

  https://doc.rust-lang.org/std/thread/
  -- https://doc.rust-lang.org/std/thread/fn.spawn.html
  -- https://doc.rust-lang.org/std/thread/struct.JoinHandle.html

  https://doc.rust-lang.org/std/sync/
  -- https://doc.rust-lang.org/std/sync/struct.Arc.html
  -- https://doc.rust-lang.org/std/sync/struct.Mutex.html
  -- https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
  -- https://doc.rust-lang.org/std/sync/struct.Condvar.html
  -- https://doc.rust-lang.org/std/sync/mpsc/
  -- https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html
  -- https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html

You can also look at the Rustonomicon:

  https://doc.rust-lang.org/nomicon/concurrency.html


### Running Rust ###

We recommend the following options to run Rust:

- Install Rust and Visual Studio Code with the rust-analyzer extension.
- Use the Rust playground online IDE (https://play.rust-lang.org/).

The advantage of the rust-analyzer extension is that it will display the types
of your variables, and you can selectively run functions if you mark them as:

```
#[test] fn foo(){...}
```

The advantage of the Rust playground is that you do not need to install an IDE.

To use the `rustc` compiler directly, you should compile with the `--test`
flag to create a test runner executable. In order to execute the test runner,
you need to run the executable, so:

  rustc --test weekN.rs && ./weekN

Alternatively, if you want to follow the proper idiomatic way of building Rust
project (for bigger projects consisting of multiple files, you really do not
want to call the compiler yourself, you want a build system for that), you can
create a Cargo project and run `cargo test`; see Chapter 1.3 and 11 of the Rust
book.

If you want to perform some output tests, you can of course also add a main
function, i.e., `fn main() { do stuff }`. To run main, you need to compile
without `--test`. */


/// Part 1: Thread spawning & Join handles

/* The following program spawns 10 threads and prints a number of messages.
Modify the program to use join handles to make sure that the main thread does
not exit until the child threads are done. */

#[test]
fn test_spawn() {
  for j in 0..10 {
    let handle = thread::spawn(move || {
      for i in 1..10 {
        println!("Hi {} from thread {}", i, j);
        thread::sleep(Duration::from_millis(100));
      }
    });
    handle.join().unwrap();
  }
}

/// Part 2: Message passing

/* Currently, the following program spawns a child thread, which sends the
vector of `[1,2,3]` to the main thread. The main thread then prints this vector.

Modify the program so that the vector is sent back and forth between the main
thread and child thread in a loop. Add one extra number to the vector each time
before it is sent (use the `vec.push(n)` method).

Explain why it is safe to mutate the vector even though it is being used by
both threads (the main thread and the child thread). */

#[test]
fn test_send_recv() {
  let (tx, rx) = mpsc::channel();
  let (tx1, rx1) = mpsc::channel();

  thread::spawn(move || {
    let mut v = vec![1,2,3];
    v.push(5);
    tx.send(v).unwrap();
    for i in 4..10 {
      let mut u: Vec<i32> = rx1.recv().unwrap();
      u.push(i);
      tx.send(u).unwrap();
    }
      
    }
  );

  for i in 14..20 {
    let mut w = rx.recv().unwrap();
    w.push(i);
    tx1.send(w).unwrap();
  }
  

  let msg = rx.recv().unwrap();
  println!("Got: {:?}", &msg);
}

// It is safe to mutate the vector because it is sent back and forth between the main
thread and child using channels



/// Part 3: Arc, Mutex & MutexGuard

/* Explain what the following program does if you uncomment the last three
lines. Insert a call to `drop(..)` to make the program print "7".  */

#[test]
fn test_mutex() {
  let m = Mutex::new(5);

  let mut num1 = m.lock().unwrap();
  *num1 = *num1 + 1;

  // let mut num2 = m.lock().unwrap();
  // *num2 = *num2 + 1;
  // println!("{}", *num2);
}

/* Complete the following program so that you spawn 10 threads that all keep
incrementing the counter in the mutex until the value exceeds 100. Print the
counter and thread number (0..10) each time it is incremented. Let the main
thread decrement the counter in a loop, and print the number each time it is
decremented. Exit if the counter reaches -100.

Question: Is this program guaranteed to terminate? If yes, why? If not, why not?
Will it terminate in practice? */

#[test]
fn test_mutex_arc() {
  let counter = Arc::new(Mutex::new(0));

  for i in 0..10 {
    // Spawn the child threads
    // Make sure that each child thread increments the mutex in a loop, and
    // prints the current value in the mutex as well as the thread number `i`.
    // Terminate the child thread when the value exceeds 100.
    unimplemented!()
  }

  loop {
    // Decrement the counter in a loop and print the current value.
    // Terminate exit the loop (with `break`) when the value goes below -100.
    unimplemented!()
  }
}


/// Part 4: Condition variables

/* In this exercise, we will implement a one-shot channel. A one-shot channel is
a channel on which you can send one message. The representation of the channel
is a `Mutex<Option<T>>`. The option will be `None` if the message has not been
sent yet, and `Some(msg)` if the message has been sent.

You will first make an implementation of the one-shot channel without condition
variables. */

// Representation of the one-shot channel in memory

struct Repr<T> {
  val: Mutex<Option<T>>,
}

// The capability held by the sender

struct Send<T> {
  repr: Arc<Repr<T>>
}

// The capability held by the receiver

struct Recv<T> {
  repr: Arc<Repr<T>>
}

// This function creates a new one-shot channel

fn new_chan<T>() -> (Send<T>, Recv<T>) {
  unimplemented!()
}

// The receiver will acquire the mutex, and check if the option is `Some(msg)`
// If it is, we will return the `msg` in the option.
// If the option is `None`, we will spin around the loop.

impl<T> Recv<T> {
  fn recv(self) -> T {
    loop {
      let mut x = self.repr.val.lock().unwrap();
      // We take the option out of the mutex and replace the value in the
      // mutex with `None`. The `option.take()` function does this for us.
      let y = x.take();
      match y {
        Some(msg) => return msg,
        None => {
          // Unlock the mutex and spin around the loop.
          drop(x)
        }
      }
    }
  }
}

// The sender acquires the mutex and stores `Some(msg)` in the mutex.

impl<T> Send<T> {
  fn send(self, msg: T) -> () {
    unimplemented!()
  }
}

// Here is a function to test the one-shot channel.

#[test]
fn test_SR() {
  let (s,r) = new_chan();
  let h = thread::spawn(move || {
    println!("Send: 10");
    thread::sleep(Duration::from_millis(1000));
    s.send(10);
    println!("Sent.");
  });
  println!("Receive.");
  let n = r.recv();
  println!("Received: {}", n);
}

/* Exercise:

The `recv()` currently uses a spin loop. This is inefficient, because it wastes
a thread and will warm up your computer. Furthermore, if the scheduler is not
fair, then a spin loop implementation is not even correct: the scheduler might
choose to keep running the spin loop, never giving the sender a chance to send
the message.

Modify the channel implemtation above to use condition variables
[`Condvar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html) to make sure
that the receive is not spinning in a hot loop when waiting to receive. Even if
the scheduler is not fair, this will be correct, because when we are waiting on
a condition variable, the scheduler must schedule some other thread. */


// Part 5: From single-shot to multi-shot

/* In this exercise, we build multi-shot channels from single-shot channels. */

// These are the representations of the receiver and sender that can be used
// to send multiple messages.

struct MultiRecv<T> {
  receiver: Recv<(T,MultiRecv<T>)>
}
struct MultiSend<T> {
  sender: Send<(T,MultiRecv<T>)>
}

// Implement this function in terms of `new_chan()` for single-shot channels.

fn new_multi_chan<T>() -> (MultiSend<T>,MultiRecv<T>) {
  unimplemented!()
}

// Implement this function in terms of the API for single-shot channels.

impl<T> MultiRecv<T> {
  fn recv(self) -> (T,MultiRecv<T>) {
    unimplemented!()
  }
}

// Implemente this function in terms of the API for single-shot channels.
// Hint: you may need to use the function `new_multi_chan()` that you previously
// defined.

impl<T> MultiSend<T> {
  fn send(self, msg: T) -> MultiSend<T> {
    unimplemented!()
  }
}

#[test]
fn test_multi_chan() {
  let (mut s,mut r) = new_multi_chan();

  thread::spawn(move ||{
    for i in 0..10 {
      println!("Send: {}", i);
      s = s.send(i);
      println!("Sent.");
    }
  });
  loop {
    println!("Receive.");
    let (msg, r2) = r.recv();
    println!("Received: {}", msg);
    r = r2;
  }
}

/* Challenge exercise:

The main thread in the program above blocks forever because there are no more
messages. Modify the example to give the sender the option to stop sending
messages, and modify the example to stop the main thread if the sender has
stopped sending messages.
Note: you have to keep using the infinite `loop { .. }`, assume that you do
not know beforehand how many messages will be sent.

In particular:

- Modify the code so that the sender has a method `drop()` in addition to
  `send()`.
- Modify the code so that the receiver's `recv()` returns an `Option`,
  indicating whether the channel has been closed.
- Modify the example `test_multi_chan` to make use of your improved `MultiSend`
  and `MultiRecv`.

Hint: modify the `MultiSend`/`MultiRecv` struct definitions to have an
`Option<...>` somewhere. */

fn main() {}
