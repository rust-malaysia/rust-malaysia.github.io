---
fonts:
  sans: 'Dejavu Sans'
  serif: 'Dejavu Serif'
  mono: 'Hack'
  provider: 'none'
---

# Error Handling

---
---

# Summary

- rust errors are explicit
  - recoverable and non-recoverable
  - types are known ahead of time (transparent)
- work well with rust data types (struct/enum)
- syntactic sugar built made it easier

---
---

# Explicit

> Explicit is better than implicit. -- python `import this`

Rust errors are explicit.

---
---

> Errors are explicit? Nani

## Example

In python help page

```python
class dict(object)
 |
 |  __getitem__(...)
 |      x.__getitem__(y) <==> x[y]
```

Does it raise exceptions?

---
---

# Types of errors

- recoverable errors
- unrecoverable errors

---
---

# Unrecoverable errors

Unexpected - `panic!`, out of bounds

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

---
---

```
                               ________________
                          ____/ (  (    )   )  \___
                         /( (  (  )   _    ))  )   )\
                       ((     (   )(    )  )   (   )  )
                     ((/  ( _(   )   (   _) ) (  () )  )
                    ( (  ( (_)   ((    (   )  .((_ ) .  )_
                   ( (  )    (      (  )    )   ) . ) (   )
                  (  (   (  (   ) (  _  ( _) ).  ) . ) ) ( )
                  ( (  (   ) (  )   (  ))     ) _)(   )  )  )
                 ( (  ( \ ) (    (_  ( ) ( )  )   ) )  )) ( )
                  (  (   (  (   (_ ( ) ( _    )  ) (  )  )   )
                 ( (  ( (  (  )     (_  )  ) )  _)   ) _( ( )
                  ((  (   )(    (     _    )   _) _(_ (  (_ )
                   (_((__(_(__(( ( ( |  ) ) ) )_))__))_)___)
                   ((__)        \\||lll|l||///          \_))
                            (   /(/ (  )  ) )\   )
                          (    ( ( ( | | ) ) )\   )
                           (   /(| / ( )) ) ) )) )
                         (     ( ((((_(|)_)))))     )
                          (      ||\(|(|)|/||     )
                        (        |(||(||)||||        )
                          (     //|/l|||)|\\ \     )
                        (/ / //  /|//||||\\  \ \  \ _)
-------------------------------------------------------------------------------
```

---
---

```text
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

---
---

# When to use non-recoverable errors?

- **non-recoverable**
- memory errors
- index errors
- impossible errors, `unwrap()` -> `expect()`
- exception, kernel

## Useful parts

- prototype code
- examples
- tests

---
---

# **Recoverable** errors

- `Result` *sum* types

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

- `Ok` for success
- `Err` for failure

---
---

# Example error

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`File::open` returns a `Result<File, io::Error>`.

---
---

# Handling result (including error)

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

When failed to open file:

```text
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

---
---

# Error, who cares?

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

---
---

# Rust cares

- due to *exhaustive pattern matching* and *sum types*

```text
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> src/main.rs:6:31
  |
6 |     let greeting_file = match greeting_file_result {
  |                               ^^^^^^^^^^^^^^^^^^^^ pattern `Err(_)` not covered
  |
note: `Result<File, std::io::Error>` defined here
 --> /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/result.rs:502:1
 ::: /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/result.rs:511:5
  |
  = note: not covered
  = note: the matched value is of type `Result<File, std::io::Error>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
7 ~         Ok(file) => file,
8 ~         Err(_) => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `playground` (bin "playground") due to previous error
```

---
---

# Matching different errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

---
---

# Shortcuts

- `unwrap` - used for no-brainer error handling, good for prototyping
  - ```rust
    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap();
    }
    ```
  - ```text
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:49
    ```
- `expect` - like unwrap but with message, more readable, use this if possible
  - ```rust
    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }
    ```
  - ```text
    thread 'main' panicked at 'hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:5:10
    ```

---
---

# Propagating errors

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

---
---

# Using the `?` operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

Or chaining method calls

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

---
---

# Syntactic sugar `?`

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("hello.txt")?;
    ...
```

Can imagine as

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    ...
```

---
---

# Patterns - custom error types

```rust
struct ConsumeError(u64);  // can contains data

fn consume(n: u64) -> Result<u64, ConsumeError> {
    if n < 10 {
        Ok(n)
    } else {
        Err(ConsumeError(n))
    }
}
```

---
---

# Patterns - wrapping (external) errors

```rust
use std::fs::File;

struct ConsumeError(u64);  // can contains data

fn consume(n: u64) -> Result<u64, ConsumeError> {
    if n < 10 {
        Ok(n)
    } else {
        Err(ConsumeError(n))
    }
}

enum ProcessError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    ConsumeError(ConsumeError),
}

fn process(f: &str) -> Result<u64, ProcessError> {
    let s = std::fs::read_to_string(f).map_err(ProcessError::IoError)?;
    let n = s.parse().map_err(ProcessError::ParseError)?;
    let n = consume(n).map_err(ProcessError::ConsumeError)?;
    Ok(n)
}
```

---
---

# Alternatives

- `Option<T>` - `Some<T>` and `None` (nullable)
- `thiserror` crate - mainly for library code
- `anyhow` crate - mainly for application code

---
---

# References

- https://doc.rust-lang.org/book/ch09-00-error-handling.html

