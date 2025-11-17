---
# presenterm
title: Rust for Everyone
author: Ivan Tham (pickfire)
theme:
  name: light
options:
  end_slide_shorthand: true
---

Rust for Everyone
=================

- talk based on Rust for Everyone! by Will Crichton https://www.youtube.com/watch?v=R0dP-QR5wQo

---

Rust is
-------

# "A language empowering everyone to build reliable and efficient software."

<!-- pause -->

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

Systems programming

<!-- column: 1 -->

Functional programming

<!-- pause -->

<!-- column: 0 -->

*(low-level machine code)*

<!-- column: 1 -->

*(high-level expressiveness)*

---

Rust is
-------

## "A language empowering everyone to build reliable and efficient software."

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

<span style="color:red">everything you don't know about</span>

<!-- column: 1 -->

<span style="color:red">everything you don't know about</span>

<!-- column: 0 -->

Systems programming

<!-- column: 1 -->

Functional programming

<!-- pause -->

<!-- column: 0 -->

<span style="color:red">numeric types, memory, pointers</span>

<!-- column: 1 -->

<span style="color:red">higher-order functions, algebraic data types, typestate</span>

---

# How can we empower more people to learn and use Rust?

---

# How can we <span style="color:red">systematically</span> empower more people to learn and use Rust?

---

<!-- jump_to_middle -->

An Ownership Visualizer
=======================

---

Example of memory violation in C++
----------------------------------

```c++
vector<int> v = {1, 2, 3}; // <--
int *num = &v[2];
v.push_back(4);
cout << *num << endl;
```

<!-- pause -->

```
stack              heap
+-----+---+        +-------+
| v   | o-|------> | 1 2 3 |
+-----+---+        +-------+
```

---

Example of memory violation in C++
----------------------------------

```c++
vector<int> v = {1, 2, 3};
int *num = &v[2]; // <--
v.push_back(4);
cout << *num << endl;
```

```
stack              heap
+-----+---+        +-------+
| v   | o-|------> | 1 2 3 |
+-----+---+        +-----^-+
| num | o-|--------------+
+-----+---+
```

---

Example of memory violation in C++
----------------------------------

```c++
vector<int> v = {1, 2, 3};
int *num = &v[2];
v.push_back(4); // <--
cout << *num << endl;
```

```
stack              heap
+-----+---+        +---------+
| v   | o-|------> | 1 2 3 4 |
+-----+---+        +---------+
| num | ? |
+-----+---+
```

---

Example of memory violation in C++
----------------------------------

```c++
vector<int> v = {1, 2, 3};
int *num = &v[2];
v.push_back(4);
cout << *num << endl; // <--
```

```
stack              heap
+-----+---+        +---------+
| v   | o-|------> | 1 2 3 4 |
+-----+---+        +---------+
| num | ? | ???
+-----+---+
```

---

Example of memory violation in C++
----------------------------------

```c++
#include <iostream>
#include <vector>
using namespace std;

int main() {
    vector<int> v = {1, 2, 3};
    int *num = &v[2];
    v.push_back(4);
    cout << *num << endl;
}
```

<!-- pause -->

```sh
$ g++ -std=c++11 vec.cpp
$ ./a.out
-1765327931
$ ./a.out
-1965683886
```

---

Rust type system prevents this
------------------------------

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    v.push(4);
    println!("{}", *num);
}
```

---

Rust type system prevents this
------------------------------

```rust
$ rustc vec.rs
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> vec.rs:4:5
  |
3 |     let num = &v[2];
  |                - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{}", *num);
  |                    ---- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

---

Intro to Ownership and friends
------------------------------

<!-- pause -->
## Ownership - one and only owner
  - transferrable
<!-- pause -->
## Borrowing
  - one mutable borrow (read-only)
  - many immutable borrow (read-write)
  - cannot have both mutable borrow and immutable borrow
<!-- pause -->
## Lifetime
  - cleaned automatically once out of scope
<!-- pause -->

Prevents memory-safety issues and *data races*.

---

Permissions model of ownership
------------------------------

![](images/aquascope.png)

---

Resources
---------

- [](https://cel.cs.brown.edu/aquascope/)
- [](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)

---

<!-- jump_to_middle -->

A Trait Debugger
================

---

Intro to Trait
--------------

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for i32 {
    fn to_string(&self) -> String { todo!() }
}

impl<S, T> ToString for (S, T)
where
    S: ToString,
    T: ToString,
{
    fn to_string(&self) -> String {
        format!(
            "({}, {})",
            self.0.to_string(),
            self.1.to_string())
    }
}
```

<!-- pause -->

<!-- column: 1 -->

```rust
fn print_items<T>(items: &[T])
where
    T: ToString,
{
    for item in items {
        println!("{}", item.to_string());
    }
}

fn main() {
    print_items(&[1, 2]);

}
```

```rust {1|1-3|1-4|all}
(i32, i32): ToString?
(i32, i32): ToString :-
  i32: ToString
i32: ToString?
i32: ToString
```

---

Intro to Trait
--------------

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust {0}
trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for i32 {
    fn to_string(&self) -> String { todo!() }
}

impl<S, T> ToString for (S, T)
where
    S: ToString,
    T: ToString,
{
    fn to_string(&self) -> String {
        format!(
            "({}, {})",
            self.0.to_string(),
            self.1.to_string())
    }
}
```

<!-- column: 1 -->

```rust {12} +line_numbers
fn print_items<T>(items: &[T])
where
    T: ToString,
{
    for item in items {
        println!("{}", item.to_string());
    }
}

fn main() {
    print_items(&[1, 2]);
    print_items(&[true, false]);
}
```

```rust
(bool, bool): ToString?
(bool, bool): ToString :-
  bool: ToString
bool: ToString? no
```

---

Show error
----------

*to-string compile error*

---

Diesel: inserting into the wrong table
--------------------------------------

![](images/diesel-error.png)

<!-- pause -->

```text
= note: the full name for the type has been written to '[…].txt'
```

---

Resources
---------

- [](https://github.com/cognitive-engineering-labs/argus)

---

<!-- jump_to_middle -->

A Program Slicer
================

---

Demo
----

---

Slices are built on aliasing and mutation
-----------------------------------------

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```python
v = ["Hello"]
s = v[0]
s += " world"
print(v)
````

<!-- pause -->

Does `print` depends on `append`?

<!-- pause -->

*no*

<!-- pause -->

```python
v = ["hello"]
v[0] += " world"
print(v)
```

<!-- pause -->

<!-- column: 1 -->

`<lhs> += <rhs>`
mutates `<lhs>`

`v[0]` refers to (or "aliases")
part of `v`

`s` do not alias `v`

<!-- pause -->

```python
v = ["hello"]
s = mystery1(v)
mystery2(s)
print(v)
```

How do we know what is in the black-box functions?

---

Ownership types enable modular slicing
--------------------------------------

```rust
let mut v = vec![String::from("hello")];

let s = v.get_mut(0).unwrap();
// how do we know get_mut returns a pointer to v?


s.push_str(" world");
// how do we know push_str mutates s?


println!("{v:?}");
```

<!-- pause -->

In C++:  `s` has type `*string`

In Rust: `s` has type `&'a mut String`

---

Ownership types enable modular slicing
--------------------------------------

```rust
let mut v = vec![String::from("hello")];

let s = v.get_mut(0).unwrap();
// how do we know get_mut returns a pointer to v?
// get_mut requires that s has the same lifetime as v

s.push_str(" world");
// how do we know push_str mutates s?


println!("{v:?}");
```

In C++:  `s` has type `*string`

In Rust: `s` has type `&'a mut String`

---

Ownership types enable modular slicing
--------------------------------------

```rust
let mut v = vec![String::from("hello")];

let s = v.get_mut(0).unwrap();
// how do we know get_mut returns a pointer to v?
// get_mut requires that s has the same lifetime as v

s.push_str(" world");
// how do we know push_str mutates s?
// push_str requires that s is a mutable reference

println!("{v:?}");
```

In C++:  `s` has type `*string`

In Rust: `s` has type `&'a mut String`

---

Resources
---------

- [Modular Information Flow through Ownership 2022](https://arxiv.org/abs/2111.13662)
- [](https://github.com/willcrichton/flowistry)

