%title: impl 3rdTrait for 3rdType?
%author: Ivan Tham
%date: 2021-05-12
%usage: mdp slide.md

-> Implementing 3rd party trait for 3rd party type <-
=====================================================

Remember our last talk (at the end)?
We talked about inline_python and how to pass data between rust and python.

---

```
            impl ToPyObject for StockRecord<'_> {
                fn to_object(&self, py: Python<'_>) -> PyObject {
                    [
                        self.day.to_string().to_object(py), // the hack
                        self.rule_id.to_object(py),
                        self.stock_id.to_object(py),
                        self.price.to_object(py),
                        self.volume.to_object(py),
                        self.open.to_object(py),
                        self.close.to_object(py),
                        self.value.to_object(py),
                        self.smi.to_object(py),
                        self.power.to_object(py),
                        self.power6m.to_object(py),
                    ]
                    .to_object(py)
                }
            }
```

---

The hack T_T
------------

- `chrono::NaiveDate` cannot be converted to `pyo3::PyObject`
- Why? `chrono::NaiveDate` does not implement `ToPyObject` (demo)
- How?

---

Let's try implementing that ourselves.

```
impl ToPyObject for chrono::NaiveDate {
    fn to_object(&self, py: Python<'_>) -> PyObject { 
        self.to_string().to_object(py)                
    }
}
```

---

> **お前はもう死んでいる。**
> (You are already dead.)

---

```
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
            
```

---

```
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
            
```

---

```
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
            
```

---

```
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
            
```

---

```
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
            
```

---

```
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
            
```

---

```
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
            
```

---

```
                           |   .:|
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
            
```


---

```
                           |    :|
                           |   .:|
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
            
```

---

```
                           |    :|
                           |    :|
                           |   .:|
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
            
            
```

---

```
                           |    :|
                           |    :|
                           |    :|
                           |   .:|
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
 
```

---


```
                           |    :|
                           |    :|
                           |    :|
                           |    :|
                           |   .:|
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
                          ``:::::''
```

---

```
                            _____
                           |    :|
                           |    :|
                           |    :|
                           |    :|
                           |   .:|
                           `...::'
                           _ ~~~ _
                        . `|     |':..
                     .     |     | `:::.
                           |     |   `:::
                   .       |     |    ::::
                           |     |    ::|::
                  .        `.   .'   ,::||:
                             ~~~     ::|||:
                   ..              .::|||:'
                    ::...       ..::||||:'
                     :::::::::::::||||::'
                      ``::::||||||||:''
```

---

```
 
 
 
 
 
 
 
 
                             ____
                     __,-~~/~    `---.
                   _/_,---(      ,    )
               __ /        <    /   )  \___
- ------===;;;'====------------------===;;;===----- -  -
                  \/  ~"~"~"~"~"~\~"~)~"/
                  (_ (   \  (     >    \)
                   \_( _ <         >_>'
                      ~ `-i' ::>|--"
                          I;|.|.|
                         <|i::|i|`.
                        (` ^'"`-' ")
```

---

> **何？**
> (What?)

---

**error[E0117]**: only traits defined in the current crate can be implemented for
arbitrary types
   --> src/main.rs:247:1
    |
247 | impl ToPyObject for chrono::NaiveDate {
    | ^^^^^^^^^^^^^^^^^^^^-----------------
    | |                   |
    | |                   _*\`NaiveDate\` is not defined in the current crate*_
    | **impl doesn't use only types from inside the current crate**
    |
    = note: define and implement a trait or new type instead

**error**: aborting due to previous error

For more information about this error, try \`rustc --explain E0117\`.
**error**: could not compile \`hello\`

To learn more, run the command again with --verbose.

(colors replicated to some extent but next time)

---

-> Orphan Rule <-
=================

^

Caused by

```
     ____  ____  ____  __  _____    _   __   ____  __  ____    ________
    / __ \/ __ \/ __ \/ / / /   |  / | / /  / __ \/ / / / /   / ____/ /
   / / / / /_/ / /_/ / /_/ / /| | /  |/ /  / /_/ / / / / /   / __/ / / 
  / /_/ / _, _/ ____/ __  / ___ |/ /|  /  / _, _/ /_/ / /___/ /___/_/  
  \____/_/ |_/_/   /_/ /_/_/  |_/_/ |_/  /_/ |_|\____/_____/_____(_)   
                                                                     
```

I repeat.

^

Before going into that ...

> https://github.com/Ixrec/rust-orphan-rules

---

-> Coherence <-
===============

Trait coherence (or coherence):

> **At most one implementation** of a trait for any given type.

Any programming language that has a feature like traits or interfaces
must either:
- Enforce coherence by simply refusing to compile programs that contain
  conflicting implementations
- Embrace incoherence and give programmers a way to manually choose an
  implementation when there's a conflict

Rust chooses to enforce coherence.

---

-> Orphan Rule <-
=================

Two rules
- "overlap rules" - no two `impl` blocks that "overlap"
  - `impl<T: Debug> Trait for T`
  - `impl<T: Display> Trait for T`
  - current workaround is through "specialization" (unstable)
- "orphan rules" - either the type or trait must be from the same crate
  - Why?
    - prevent "dependency hell", what if both crates have the same impl?
    - allow `impl` to be added without being a breaking change
  - In our case, `pyo3` and `chrono` both comes from different crates

---

-> So how? <-
=============

Note that I only show some solutions that was done here.

- newtype (chrono-pyo3 crate)
  - https://github.com/kangalioo/pyo3-chrono
  - easy solution, just `struct NaiveDate(NaiveDate)`
  - then just delegate everything but add an additional `impl`
  - may not be good for popular/core crates like pyo3
- integrate it directly with the library (we will discuss this)
  - WIP https://github.com/chronotope/chrono/pull/542
  - can utilize underlying data (implementation details) directly
  - in this case, troublesome since MSRV for chrono is low

---

-> Change chrono upstream <-
============================

Why not change pyo3?

I started out with chrono and think it is a good idea.

But basically could be added to either side.

---

-> Cargo features <-
====================

Conditional compilation for pyo3 integration, using crates features flag.

```
[dependencies]
libc = { version = "0.2.69", optional = true }
time = { version = "0.1.43", optional = true }
num-integer = { version = "0.1.36", default-features = false }
num-traits = { version = "0.2", default-features = false }
rustc-serialize = { version = "0.3.20", optional = true }
serde = { version = "1.0.99", default-features = false, optional = true }
pure-rust-locales = { version = "0.5.2", optional = true }
pyo3 = { version = "0.13.2", optional = true }
```
 \^ implicit feature infered from dependency I guess

-> Usage <-
-----------

```
[dependencies]
chrono = { version = "0.4", features = ["pyo3"] }
```

---

-> Code <-
==========

> Talk is cheap. Show me the code. -- Linus Torvalds

```
#[cfg(feature = "pyo3")]
mod pyo3 {
    use ...;
 
    impl ToPyObject for NaiveDate {
        fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
            let mdf = self.mdf();
            let date = PyDate::new(py, self.year(), mdf.month() as u8, mdf.day() as u8)
                .expect("Failed to construct date");
            date.into()
        }
    }
    impl IntoPy<pyo3::PyObject> for NaiveDate { ... }
    impl FromPyObject<'_> for NaiveDate { ... }
}
```

---

-> Progress <-
==============

Pull request still not done yet due to pyo3 upstream missing python
datetime FFI components for timezone.

https://github.com/PyO3/pyo3/pull/1588

That is way harder to talk about and probably not for meetup. T_T

---

-> Q&A <-
=========

