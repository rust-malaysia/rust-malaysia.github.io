---
layout: post
title:  The Burger Problem
date:   2019-12-08 22:49:37 +0800
categories: code
---

tl;dr racing with cancellation and timeout

Malaysians have an interesting queuing mechanism, instead of having one line
for counters, there is one line for each counter, each counter have different
speeds so people try to queue on multiple counter at once and see who reach
first. Let us see the illustration:

    counter 1 | . . . . a
    counter 2 | . . . . b
    counter 3 | . . . . . c

    counter 1 | . . . . a
    counter 2 | . . . b
    counter 3 | . . . . c

    counter 1 | . . . . a
    counter 2 | . . . b
    counter 3 | . . c

    counter 1 | . . . a
    counter 2 | . b
    counter 3 | . . c

    counter 1 | . . .
    counter 2 | b
    counter 3 | . .

A group of 3 (a, b, c) starts queuing on 3 counter (1, 2, 3) to buy burger
(vegan I wished), looking at the illustration, they start queuing at the same
time but counter 2 is faster so b reached the counter first, the rest of the
group then stop queuing.

Of course, the scenario shown above is the happy scenario. Here are the others:

1. One of the three reaches the counter first, so the other two stops waiting
   at the line.

2. Two or more of them reaches the counter at nearly the same time. Only one of
   them buys the burgers for everyone. Other exits the line.

3. All of them timed out, since the wait is too long, and goes elsewhere. No
   burgers. T_T

The predetermined random wait time per "worker" is just a shorthand, since the
correct algorithm is probably you have to check if your line moves at every
"tick". Or simulate the whole thing with some lines "faster" than the others.

It's a classic race for resources. You have just 3 workers instead of yourself
to hedge your bets to avoid the worse case scenarios.

I wish to write test but not sure how test can be written. Feel free to
contribute. The code will be written step-by-step, feel free to skip to the end
for the last source code.

I was thinking of using channels in this case but channel does not seem to
work. Looking at `std::sync` documentation, there is
[`std::sync::CondVar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html)
which seems complicated but may be helpful, it allows blocking a `Mutex` until
some event occurred.

Let's steal the example and do some modification.

```rust
use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

fn main() {
    let mut handles = Vec::new();
    let pair = Arc::new((Mutex::new(()), Condvar::new()));

    for i in 0..3 {
        let pair = pair.clone();
        handles.push(thread::spawn(move || {
            let dur = Duration::from_secs(rand::thread_rng().gen_range(1, 6));
            let (ref lock, ref cvar) = *pair;
            let (_, ended) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
            println!("Worker {} {:?} {:?}", i, dur, ended.timed_out());
            cvar.notify_all();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

For each thread, it creates a random duration to simulate the "wait" and use
`CondVar` to notify all threads blocking on `CondVar`, this example also
provides a wait timeout in case it takes too long. `wait_timeout` on `CondVar`
waits for notification on a `MutexGuard` (can be obtained from a `Mutex`) and a
timeout, blocking the thread for a duration.

Take note, there is an issue here where the race is not handled, scenario 1
works here but scenario 2 is broken here such that both worker may reach the
counter and both buys food (only one should buy).

Side note, writing this in a functional manner using iterators (`Iter`).

```rust
use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

fn main() {
    let pair = Arc::new((Mutex::new(()), Condvar::new()));

    (0..3)
        .map(|i| {
            let pair = pair.clone();
            thread::spawn(move || {
                let dur = Duration::from_secs(rand::thread_rng().gen_range(1, 6));
                let (ref lock, ref cvar) = *pair;
                let (_, ended) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
                println!("Worker {} {:?} {:?}", i, dur, ended.timed_out());
                cvar.notify_all();
            })
        })
        .for_each(|handle| handle.join().unwrap());
}
```

Later we have also tried using [`bus`](https://crates.io/crates/bus) and
[`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)
channels, there is this fan-in and fan-out thing but it does not solve scenario
2, the code looks weird so I stop making this method work.

```rust
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

fn main() {
    let mut handles = Vec::new();
    let pair = Arc::new((Mutex::new(AtomicBool::new(true)), Condvar::new()));

    for i in 1..=3 {
        let pair = pair.clone();
        handles.push(thread::spawn(move || {
            let dur = Duration::from_secs(rand::thread_rng().gen_range(1, 6));
            let (ref lock, ref cvar) = *pair;
            let (ended, _) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
            cvar.notify_all();
            if ended.swap(false, Ordering::Relaxed) {
                println!("Worker {} {:?} done", i, dur);
            } else {
                println!("Worker {} {:?}", i, dur);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

Looking at the documentation, we could make using of
[`AtomicBool`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html)
since we only need one of the first that reaches the counter,
`std::sync::atomic` is thread safe since it implements
[`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) so we can use it
to track atomic values between threads, only one shall prevail. This solve
scenario 2 but we still have scenario 3. Hooray!

We track `AtomicBool::new(true)` in the `Mutex` together with `CondVar`, making
sure that only one thread is reading from it any time. With `true` indicating
that no worker reached the counter yet, we use
[`swap`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.swap)
to write `false` to the `AtomicBool`, letting others know that someone reached
the counter just in case both reached at the same time.

`swap` returns the previous value which we could use to check the last value,
with `ended.swap(false, Ordering::Relaxed)` it allows us to make sure only the
first worker will reach that condition. `Ordering::Relaxed` is the memory
ordering of the operation and is out of scope of this article.

Let us not stop there, improving the code we could have just `notify_all` once
for the first worker instead of having multiple workers `notify_all`.

```rust
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

fn main() {
    let mut handles = Vec::new();
    let pair = Arc::new((Mutex::new(AtomicBool::new(true)), Condvar::new()));

    for i in 1..=3 {
        let pair = pair.clone();
        handles.push(thread::spawn(move || {
            let dur = Duration::from_secs(rand::thread_rng().gen_range(1, 6));
            let (ref lock, ref cvar) = *pair;
            let (ended, _) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
            if ended.swap(false, Ordering::Relaxed) {
                cvar.notify_all();
                println!("Worker {} {:?} done", i, dur);
            } else {
                println!("Worker {} {:?}", i, dur);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

How do we do scenario 3 where we wanted the workers to stop in case the
counters are too slow (also true in real life, Malaysia counters may be
slow/inefficient to the point that customers leave)? Oh, just add another
thread to do the time keeping.

```rust
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

fn main() {
    let mut handles = Vec::new();
    let pair = Arc::new((Mutex::new(AtomicBool::new(true)), Condvar::new()));

    for i in 1..=3 {
        let pair = pair.clone();
        handles.push(thread::spawn(move || {
            let dur = Duration::from_secs(rand::thread_rng().gen_range(1, 6));
            let (ref lock, ref cvar) = *pair;
            let (ended, _) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
            if ended.swap(false, Ordering::Relaxed) {
                cvar.notify_all();
                println!("Worker {} {:?} done", i, dur);
            } else {
                println!("Worker {} {:?}", i, dur);
            }
        }));
    }

    thread::spawn(move || {
        let dur = Duration::from_secs(3);
        let (ref lock, ref cvar) = *pair;
        let (ended, _) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
        if ended.swap(false, Ordering::Relaxed) {
            cvar.notify_all();
            println!("Timed out {:?}", dur);
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }
}
```

Here the new thread stops all the worker in case they take too long (3 seconds
in this case, still rare enough), it also prints out a nice message for timeout
in addition to stopping any worker from showing done.

After writing and looking at the documentation for explanation, I just realized
that `AtomicBool` was duplicating the job of `Mutex` so using plain `bool` is
enough, we have to make the `MutexGuard` mutable and modify the value in
`Mutex` through `Deref` (`*`). Latest code:

```rust
use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};

fn main() {
    let mut handles = Vec::new();
    let pair = Arc::new((Mutex::new(true), Condvar::new()));

    for i in 1..=3 {
        let pair = pair.clone();
        handles.push(thread::spawn(move || {
            let dur = Duration::from_secs(rand::thread_rng().gen_range(1, 6));
            let (ref lock, ref cvar) = *pair;
            let (mut ended, _) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
            if *ended {
                *ended = false;
                cvar.notify_all();
                println!("Worker {} {:?} done", i, dur);
            } else {
                println!("Worker {} {:?}", i, dur);
            }
        }));
    }

    thread::spawn(move || {
        let dur = Duration::from_secs(3);
        let (ref lock, ref cvar) = *pair;
        let (mut ended, _) = cvar.wait_timeout(lock.lock().unwrap(), dur).unwrap();
        if *ended {
            *ended = false;
            cvar.notify_all();
            println!("Timed out {:?}", dur);
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }
}
```

All three scenarios have been solved up to this point, utilizing `std::sync`,
`Mutex` and atomics. I found it hard at first to understand `CondVar` in this
case but it became easy after playing with it.

We thank Ang for showing the question, it was also implemented in Go and Elixir
in our telegram group (lazy to put here).
