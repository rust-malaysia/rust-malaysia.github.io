---
layout: post
title:  Faster Integer Parsing
date:   2020-07-12 01:11:27 +0800
categories: code
---

tl;dl Rust port of
<https://kholdstare.github.io/technical/2020/05/26/faster-integer-parsing.html>
to parse fixed 16 integer.

Note that a lot of the text are taken from the original article along with my
additional comments but the code will be written as Rust.

# The problem

Let's say, theoretically, you have some text-based protocol, or file that
contains microsecond timestamps. You need to parse these timestamps as quickly
as possible. Maybe it is JSON, maybe it is a CSV file, maybe something else
bespoke. It's 16 characters long, and this could also apply to credit card
numbers.

    timestamp,event_id
    1585201087123567,a
    1585201087123585,b
    1585201087123621,c

In the end you have to implement a function similar to this:

```rust
fn parse_timestamp(s: &str) -> u64 {
    todo!()
}
```

# The preparation

We will be using nightly to be able to use the feature `test` which comes with
the nightly release. Use `rustup toolchain add nightly` to install the nigthly
toolchain.

We will be creating the project by using `cargo new parseint --lib` to create a
new library. Next, we will specify that we will be using the nightly toolchain
for this crate by `echo nightly > rust-toolchain`.

Our `benches/bench.rs` will be something like the following.

```rust
#![feature(test)]
extern crate test;

use parseint::*;
use test::{black_box, Bencher};

const EXAMPLE_TIMESTAMP: &str = "1585201087123789";
const EXPECTED_TIMESTAMP: u64 = 1585201087123789;
```

Every time we create a new function for benchmark, we will append the following
block to `benches/bench.rs` prepended by `bench_` to keep naming simple. Say we
have a function `str_parse` (coming up next), so it will not be written later.

```rust
#[bench]
fn bench_str_parse(b: &mut Bencher) {
    assert_eq!(str_parse(EXAMPLE_TIMESTAMP), EXPECTED_TIMESTAMP);
    b.bytes = EXAMPLE_TIMESTAMP.len() as u64;
    b.iter(|| str_parse(black_box(EXAMPLE_TIMESTAMP)));
}
```

Having the `assert_eq` is a safety measure that our functions live up to its'
expectations. `b.bytes` allows us to have nice throughput output when we run
`cargo bench` to benchmark later. `black_box` prevent the compiler from
optimizing the test code which may affect the result.

# The native solution

Let us start with what is available. We have `str::parse` from standard library
which acts as a baseline for the comparison later.

From now on, we will only be showing the code be write in `src/lib.rs`, note
that we will also add code to `bench.rs` as shown above but we will not specify
it here.

```rust
pub fn str_parse(s: &str) -> u64 {
    s.parse().unwrap()
}
```

Note that comparing it to `str::parse` with the other functions is not fair
since it is design to validate and handle numbers with different length. And I
would say it the standard library version is already quite optimized. One can
look into the assembly code by using `cargo asm parseint::str_parse --rust`.

```
 pub fn str_parse(s: &str) -> u64 {
 sub     rsp, 24
 mov     rdx, rsi
 mov     rsi, rdi
 lea     rdi, [rsp, +, 8]
     FromStr::from_str(self) (libcore/str/mod.rs:4331)
     call    qword, ptr, [rip, +, _ZN4core3num52_$LT$impl$u20$core..str..FromStr$u20$for$u20$u64$GT$8from_str17hb6ec24c4ee2a5da7E@GOTPCREL]
     Ok(t) => t, (libcore/result.rs:1004)
     cmp     byte, ptr, [rsp, +, 8], 1
     je      .LBB5_1
     Ok(t) => t, (libcore/result.rs:1004)
     mov     rax, qword, ptr, [rsp, +, 16]
 }
 add     rsp, 24
 ret
.LBB5_1:
     Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e), (libcore/result.rs:1005)
     mov     al, byte, ptr, [rsp, +, 9]
     mov     byte, ptr, [rsp, +, 7], al
     Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e), (libcore/result.rs:1005)
     lea     rdi, [rip, +, .L__unnamed_2]
     lea     rcx, [rip, +, .L__unnamed_1]
     lea     r8, [rip, +, .L__unnamed_3]
     lea     rdx, [rsp, +, 7]
     mov     esi, 43
     call    qword, ptr, [rip, +, _ZN4core6result13unwrap_failed17h9507895bc765c906E@GOTPCREL]
     ud2
```

Running `cargo bench` gives us.

```
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
```

Since we know our string contains the number we are trying to parse, and we do
not need to do any whitespace skipping, can we be faster? Just how much time is
spent in validation?

# The naive solution

Let us write a good old for loop. Read the string character by character, and
build up the result.

```rust
pub fn naive_chars(s: &str) -> u64 {
    let mut result = 0;
    for digit in s.chars() {
        result *= 10;
        result += digit as u64 - '0' as u64;
    }
    result
}
```

```
test bench_naive_chars         ... bench:          18 ns/iter (+/- 0) = 888 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
```

We can see that the naive version is already slightly faster (~27%) than the
original version.

In the Rust official book, it was said to produce better performing code. Let us
try that out here.

```rust
pub fn naive_chars_iter(s: &str) -> u64 {
    s.chars().fold(0, |a, c| a * 10 + c as u64 - '0' as u64)
}
```

It does not produce code which performs faster here but the code now is shorter
and slightly easier to read in my own opinion.

Somewhere, during a conversion in `Rust 众`, a Rust telegram group with people
mainly from China (Taiwan have a separate group `rust.tw`). Someone mentioned
that it may be faster to get the value of a number by using bitwise and. Looking
at the ASCII table (`man ascii`), noticed that it all ends with the same number.

       2 3 4 5 6 7       30 40 50 60 70 80 90 100 110 120
     -------------      ---------------------------------
    0:   0 @ P ` p     0:    (  2  <  F  P  Z  d   n   x
    1: ! 1 A Q a q     1:    )  3  =  G  Q  [  e   o   y
    2: " 2 B R b r     2:    *  4  >  H  R  \  f   p   z
    3: # 3 C S c s     3: !  +  5  ?  I  S  ]  g   q   {
    4: $ 4 D T d t     4: "  ,  6  @  J  T  ^  h   r   |
    5: % 5 E U e u     5: #  -  7  A  K  U  _  i   s   }
    6: & 6 F V f v     6: $  .  8  B  L  V  `  j   t   ~
    7: ' 7 G W g w     7: %  /  9  C  M  W  a  k   u  DEL
    8: ( 8 H X h x     8: &  0  :  D  N  X  b  l   v
    9: ) 9 I Y i y     9: '  1  ;  E  O  Y  c  m   w
    A: * : J Z j z
    B: + ; K [ k {
    C: , < L \ l |
    D: - = M ] m }
    E: . > N ^ n ~
    F: / ? O _ o DEL

Here, we start from the digit `0`, decimal `48`, octal `30`, binary `110000`.
Going up to `9`, decimal `57`, octal `39`, binary `111001`. Noticed that the
octal version and binary version have the digits at the last byte (`0000` -
`1001`), we could have do a `& 0x0f` to get the last 4 bits.

Being naive here, we just ignore the 5 possible values after `9` in the ASCII
table. Let us try to add this and see if it is faster.

```rust
pub fn naive_chars_and(s: &str) -> u64 {
    s.chars().fold(0, |a, c| a * 10 + (c as u8 & 0x0f) as u64)
}
```

```
test bench_naive_chars         ... bench:          18 ns/iter (+/- 0) = 888 MB/s
test bench_naive_chars_and     ... bench:          17 ns/iter (+/- 1) = 941 MB/s
test bench_naive_chars_iter    ... bench:          18 ns/iter (+/- 1) = 888 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
```

Nice, 1ns improvement, quite good for just an operator change from minus to
bitwise and.

# The bytes solution

When someone started to explore parsing and string manipulation in Rust. It is
likely that one would have heard that iterating on bytes would be faster than
iterating on characters.

This is because Rust `String` is always UTF-8 encoded, `String::chars` is
iterating on `char` which represents a Unicode Scalar Value. But we know that
`0` to `9` in ASCII will always be a single byte. `String::bytes` which returns
an iterator on `u8` would work for us. I bet this is faster, let us add that.

```rust
pub fn naive_bytes(s: &str) -> u64 {
    let mut result = 0;
    for digit in s.bytes() {
        result *= 10;
        result += (digit - b'0') as u64;
    }
    result
}

pub fn naive_bytes_iter(s: &str) -> u64 {
    s.bytes().fold(0, |a, c| a * 10 + (c - b'0') as u64)
}

pub fn naive_bytes_and(s: &str) -> u64 {
    s.bytes().fold(0, |a, c| a * 10 + (c & 0x0f) as u64)
}
```

It involves very simple change, by changing `chars` to `bytes`, and `'0'` to
`b'0'`. But doing this improves the benchmark by another 70%, huge drop of 7ms.
We have now reduced more than half the time from the baseline.

```
test bench_naive_bytes         ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_chars         ... bench:          18 ns/iter (+/- 0) = 888 MB/s
test bench_naive_chars_and     ... bench:          17 ns/iter (+/- 1) = 941 MB/s
test bench_naive_chars_iter    ... bench:          18 ns/iter (+/- 1) = 888 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
```

# The anti bound checking solution

After completing the port (including the parts below), I share the results to
[simd-json issues](https://github.com/simd-lite/simd-json/issues/132), @Licenser
did a review and gave some feedback to improve the performance. One of them is
to reduce bound checking by specifying the number of bytes to take. I am
surprised that this can further improve the results of `naive_bytes_and`.

```rust
pub fn naive_bytes_and_c16(s: &str) -> u64 {
    s.bytes()
        .take(16)
        .fold(0, |a, c| a * 10 + (c & 0x0f) as u64)
}
```

It does look obvious to me that we are working with 16 characters but the
compiler does not know this. I really hoped that bound checking can be removed
later so that users (if there is a way) do not need to do godbolting to reduce
the bound checking. We can do this by specifying `take` on `Iterator`. I am
surprised that this improves the timing by another 2ns (20%).

```
test bench_naive_bytes         ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_chars         ... bench:          18 ns/iter (+/- 0) = 888 MB/s
test bench_naive_chars_and     ... bench:          17 ns/iter (+/- 1) = 941 MB/s
test bench_naive_chars_iter    ... bench:          18 ns/iter (+/- 1) = 888 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
```

# The brute force solution

If we know it is 16 bytes, why even have a for loop? Let us unroll it!

```rust
pub fn unrolled(s: &str) -> u64 {
    let mut result = 0;
    let bytes = s.as_bytes();
    result += (bytes[0] - b'0') as u64 * 1000000000000000;
    result += (bytes[1] - b'0') as u64 * 100000000000000;
    result += (bytes[2] - b'0') as u64 * 10000000000000;
    result += (bytes[3] - b'0') as u64 * 1000000000000;
    result += (bytes[4] - b'0') as u64 * 100000000000;
    result += (bytes[5] - b'0') as u64 * 10000000000;
    result += (bytes[6] - b'0') as u64 * 1000000000;
    result += (bytes[7] - b'0') as u64 * 100000000;
    result += (bytes[8] - b'0') as u64 * 10000000;
    result += (bytes[9] - b'0') as u64 * 1000000;
    result += (bytes[10] - b'0') as u64 * 100000;
    result += (bytes[11] - b'0') as u64 * 10000;
    result += (bytes[12] - b'0') as u64 * 1000;
    result += (bytes[13] - b'0') as u64 * 100;
    result += (bytes[14] - b'0') as u64 * 10;
    result += (bytes[15] - b'0') as u64;
    result
}
```

The original article shows that unrolling makes the code faster but it looks
like the compiler is more clever than we thought, it already auto-vectorizes the
code before this. 16ms! It is way slower than any implementation using bytes,
we can see this with those `movzx` in `cargo asm parseint::naive_bytes_and_c16`.

```
test bench_naive_bytes         ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_chars         ... bench:          18 ns/iter (+/- 0) = 888 MB/s
test bench_naive_chars_and     ... bench:          17 ns/iter (+/- 1) = 941 MB/s
test bench_naive_chars_iter    ... bench:          18 ns/iter (+/- 1) = 888 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
test bench_unrolled            ... bench:          16 ns/iter (+/- 0) = 1000 MB/s
```

I do no trust my eyes looking at this. I heard others that they always blame the
safe code and using `unsafe` makes it faster. `get_unchecked` is a simple
`unsafe` and we cannot go wrong by using it here right?

```rust
pub fn unrolled_unsafe(s: &str) -> u64 {
    let mut result = 0;
    let bytes = s.as_bytes();
    result += (unsafe { bytes.get_unchecked(0) } - b'0') as u64 * 1000000000000000;
    result += (unsafe { bytes.get_unchecked(1) } - b'0') as u64 * 100000000000000;
    result += (unsafe { bytes.get_unchecked(2) } - b'0') as u64 * 10000000000000;
    result += (unsafe { bytes.get_unchecked(3) } - b'0') as u64 * 1000000000000;
    result += (unsafe { bytes.get_unchecked(4) } - b'0') as u64 * 100000000000;
    result += (unsafe { bytes.get_unchecked(5) } - b'0') as u64 * 10000000000;
    result += (unsafe { bytes.get_unchecked(6) } - b'0') as u64 * 1000000000;
    result += (unsafe { bytes.get_unchecked(7) } - b'0') as u64 * 100000000;
    result += (unsafe { bytes.get_unchecked(8) } - b'0') as u64 * 10000000;
    result += (unsafe { bytes.get_unchecked(9) } - b'0') as u64 * 1000000;
    result += (unsafe { bytes.get_unchecked(10) } - b'0') as u64 * 100000;
    result += (unsafe { bytes.get_unchecked(11) } - b'0') as u64 * 10000;
    result += (unsafe { bytes.get_unchecked(12) } - b'0') as u64 * 1000;
    result += (unsafe { bytes.get_unchecked(13) } - b'0') as u64 * 100;
    result += (unsafe { bytes.get_unchecked(14) } - b'0') as u64 * 10;
    result += (unsafe { bytes.get_unchecked(15) } - b'0') as u64;
    result
}
```

Oh no, `unsafe` makes it worse. I guess it is not always right that
`get_unchecked` makes stuff faster.

```
test bench_unrolled            ... bench:          16 ns/iter (+/- 0) = 1000 MB/s
test bench_unrolled_unsafe     ... bench:          17 ns/iter (+/- 0) = 941 MB/s
```

Let us apply the previous trick to reduce bounds checking here. We could use
`get(..16)` on `&str` like `take` on `Iterator`.

```rust
pub fn unrolled_safe(s: &str) -> u64 {
    let mut result = 0;
    let bytes = s.get(..16).unwrap().as_bytes();
    result += (bytes[0] - b'0') as u64 * 1000000000000000;
    result += (bytes[1] - b'0') as u64 * 100000000000000;
    result += (bytes[2] - b'0') as u64 * 10000000000000;
    result += (bytes[3] - b'0') as u64 * 1000000000000;
    result += (bytes[4] - b'0') as u64 * 100000000000;
    result += (bytes[5] - b'0') as u64 * 10000000000;
    result += (bytes[6] - b'0') as u64 * 1000000000;
    result += (bytes[7] - b'0') as u64 * 100000000;
    result += (bytes[8] - b'0') as u64 * 10000000;
    result += (bytes[9] - b'0') as u64 * 1000000;
    result += (bytes[10] - b'0') as u64 * 100000;
    result += (bytes[11] - b'0') as u64 * 10000;
    result += (bytes[12] - b'0') as u64 * 1000;
    result += (bytes[13] - b'0') as u64 * 100;
    result += (bytes[14] - b'0') as u64 * 10;
    result += (bytes[15] - b'0') as u64;
    result
}
```

Looks like this is more worthwhile than using `unsafe` it reduces the timing by
1ms rather than increasing it by 1ms.

```
test bench_unrolled            ... bench:          16 ns/iter (+/- 0) = 1000 MB/s
test bench_unrolled_safe       ... bench:          15 ns/iter (+/- 1) = 1066 MB/s
test bench_unrolled_unsafe     ... bench:          17 ns/iter (+/- 0) = 941 MB/s
```

Still, loop unrolling does not help like it did on the original article. Time
too look into some other trick.

# The byteswap insight

Let us draw out the operations in the unrolled solution as a tree, on a
simplified example of parsing '1234' into a 32-bit integer:

    ('1' - '0') * 1000 = 1000
    ('2' - '0') *  100 =  200
    ('3' - '0') *   10 =   30
    ('4' - '0') *    1 =    4

      1000
    +  200
    = 1200
    +   30
    = 1230
    +    4
    = 1234

    The original version have an image of a graph, but I like ASCII art more

We can see that the amount of multiplications and additions is linear with the
amount of characters. It is hard to see how to improve this, because every
multiplication is by a different factor (so we cannot multiple "in one go"),
and at the end of the day we need to add up all the intermediate results.

However, it is still very regular. For one thing, the first character in the
string is multiplied by the largest factor, because it is the most significant
digit.

> On a little-endian machine (like x86), an integer's first byte contains the
> least significant digits, while the first byte in a string contains the most
> significant digit.

      "1234" (octal 0x34333231)
    - "0000" (octal 0x30303030)
    =               0x04030201
      (byteswap) -> 0x01020304

    Looking at the string as an integer we can get closer to the final parsed
    state in fewer operations - see how the hex representation is almost what we
    want

Now to reinterpret the bytes of a string as an integer we have to use
`std::ptr::copy_nonoverlapping` (similar to `memcpy` in C) and we have to use
`u64::swap_bytes` to swap the bytes in one instruction.

Note the code below will not be used, just for demonstration and proof of
concept for the above. It was removed so this came from my mind, random code.

```rust
// dangerous act, kids should not try this at home - 眼看手不动、手动屁股痛
fn parse_8_rubbish(s: &str) -> u64 {
    let s = s.as_ptr() as *const _;
    let mut chunk: u64 = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(s, &mut chunk, std::mem::size_of_val(&chunk));
    }

    let mut zeros = 0;
    let z = "0000".as_ptr() as *const _;
    unsafe {
        std::ptr::copy_nonoverlapping(z, &mut zeros, std::mem::size_of_val(&chunk));
    }

    chunk = (chunk - zeros).swap_bytes();
    todo!()
}
```

But now that we have an integer that kind of, sort of looks like what we want,
how do we get it across the finish line without too much work?

# The divide and conquer insight

From the previous step, we end up with an integer whose bit representation has
each digit placed in a separate byte. Even though one byte can represent up to
256 values, we have values 0-9 in each byte of the integer. They are also in the
right little endian order. Now we just need to "smash" them together somehow.

We know that doing it linearly would be too slow, what is the next possibility?
`O(log(n))`! We need to combine every adjacent digit into a pair in one step,
and then each pair of digits into a group of four, and so on, until we have the
entire integer.

> The key is working on adjacent digits simultaneously. This allows a tree of
> operations, running in O(log(n)) time.

This involves multiplying the even-index digits by a power of 10 and leaving the
odd-index digits alone. This can be done with bitmasks to selectively apply
operations.

                             0x04030201
    +-----------------------------|--------------------+
    | 1 byte mask trick           v  v--- one byte     |
    |                         +-+-+-+-+                |
    |                         |1|2|3|4|                |
    |                         +-+-+-+-+                |
    |                           |   |                  |
    |            0x000f000f  <--+   +-->  0x0f000f00   |
    |                 |                        |       |
    |                 v                        v       |
    |               0 2 0 4                  1 0 3 0   |
    |       (>> 8)  2 0 4 0          (* 10) 10 030 0   |
    |                 |                        |       |
    |                 +-----------+------------+       |
    |                         (+) |                    |
    |                             v v--- two bytes     |
    |                          +--+--+                 |
    |                          |12|34|                 |
    |                          +--+--+                 |
    +-----------------------------|---------------------
                                  |
                        +--------------------+
                        | 2 bytes mask trick |
                        +--------------------+
                                  |
                                  v  v--- four bytes
                               +------+
                               | 1234 |
                               +------+

    By using bitmasking, we can apply operations to more than one digit at a
    time, to combine them into a larger group

Let us finish the `parse_8_rubbish` function we started earsier by employing
this masking trick. As a neat side-effect of the masking, we do not need to
subtract '0', since it will be masked away.

```rust
fn parse_8_chars(s: &str) -> u64 { // no need to benchmark this, to be used later
    let s = s.as_ptr() as *const _;
    let mut chunk = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(s, &mut chunk, std::mem::size_of_val(&chunk));
    }

    // 1-byte mask trick (works on 4 pairs of single digits)
    let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f) * 10;
    let chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on 2 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff) * 100;
    let chunk = lower_digits + upper_digits;

    // 4-byte mask trick (works on a pair of four digits)
    let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff) * 10000;
    let chunk = lower_digits + upper_digits;

    chunk
}
```

# The trick

Putting it all together, to parse our 16-digit integer, we break it up into two
chunks of 8 bytes, run parse_8_chars that we have just written and benchmark it!

```rust
pub fn trick(s: &str) -> u64 {
    let (upper_digits, lower_digits) = s.split_at(8);
    parse_8_chars(upper_digits) * 100000000 + parse_8_chars(lower_digits)
}
```

Let us strip some extra information from the benchmarks with the useful ones.

```
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
test bench_unrolled_safe       ... bench:          15 ns/iter (+/- 1) = 1066 MB/s
test bench_trick               ... bench:           6 ns/iter (+/- 0) = 2666 MB/s
```

Nice! Looks like this method that we pulled off is able to shave off yet another
2ms (25%) over the previous fastest method `naive_bytes_and_c16`. How about we
let CPU do all the hard work like in the original article?

# The SIMD trick

We have the main insight:

- Combine groups of digits simultaneously to archive `O(log(n))` time

We also have a 16-characters, or 128-bit string to parse - can we use SIMD? Of
course we can! [SIMD stands for Single Instruction Multiple
Data](https://en.wikipedia.org/wiki/SIMD), and is exactly what we are looking
for. SSE and AVX instructions are supported on both Intel and AMD CPUs, and they
typically work with wider registers. (and even on my 10 years old laptop)

Rust do provide instrinsics in `std::arch` and we need to use `unsafe` to use
them, the functions in the documentations will direct you to [Intel Intrinsics
Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/) itself.

Let us set up the digits in each of the 16 bytes first:

```rust
pub fn trick_simd(s: &str) -> u64 {
    unsafe {
        let chunk = _mm_lddqu_si128(std::mem::transmute_copy(&s));
        let zeros = _mm_set1_epi8(b'0' as i8);
        let chunk = _mm_sub_epi16(chunk, zeros);

        // ...
    }
}
```

And now, the star of the show is the `madd` functions. These SIMD functions do
exactly what we did with our bitmask tricks - they take a wide register,
interpret it as a vector of smaller integers, multiply each by a given
multiplier, and add neighboring ones together into a vector of wider integers.
All in one instructions!

As an example of taking every byte, multiplying the odd ones by 10 and adding
adjacent pairs together, we can use `_mm_add_epi16`.


```rust
        // previous code

        // The 1-byte "trick" in one instruction
        let mult = _mm_set_epi8(1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10);
        let chunk = _mm_maddubs_epi16(chunk, mult);

        // ...
```

There is another instruction for the 2-byte trick, but unfortunately I could not
find one for the 4-byte trick - that needed two instructions. Here is the
completed `parse_simd` to parse 16 characters:

```rust
use core::arch::x86_64::{
    _mm_cvtsi128_si64, _mm_lddqu_si128, _mm_madd_epi16, _mm_maddubs_epi16, _mm_packus_epi32,
    _mm_set1_epi8, _mm_set_epi16, _mm_set_epi8, _mm_sub_epi16,
};

pub fn trick_simd(s: &str) -> u64 {
    unsafe {
        let chunk = _mm_lddqu_si128(std::mem::transmute_copy(&s));
        let zeros = _mm_set1_epi8(b'0' as i8);
        let chunk = _mm_sub_epi16(chunk, zeros);

        let mult = _mm_set_epi8(1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10);
        let chunk = _mm_maddubs_epi16(chunk, mult);

        let mult = _mm_set_epi16(1, 100, 1, 100, 1, 100, 1, 100);
        let chunk = _mm_madd_epi16(chunk, mult);

        let chunk = _mm_packus_epi32(chunk, chunk);
        let mult = _mm_set_epi16(0, 0, 0, 0, 1, 10000, 1, 10000);
        let chunk = _mm_madd_epi16(chunk, mult);

        let chunk = _mm_cvtsi128_si64(chunk) as u64;
        ((chunk & 0xffffffff) * 100000000) + (chunk >> 32)
    }
}
```

```
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
test bench_unrolled_safe       ... bench:          15 ns/iter (+/- 1) = 1066 MB/s
test bench_trick               ... bench:           6 ns/iter (+/- 0) = 2666 MB/s
test bench_trick_simd          ... bench:           8 ns/iter (+/- 1) = 2000 MB/s
```

8 nanoseconds! Ugly. (2ns drop, same as `bench_naive_bytes_and_c16`)

The original article have SIMD version performing better than the trick version,
but seemed like SIMD perform worse here. Maybe because of my old laptop. But
still it is quite surprising to see SIMD code slower than non-SIMD.

Remembering what my dad always said in Cantonese 點有咁大隻蛤乸隨街跳呀？ In
English, it means how can you ever find a big clam jumping randomly on the
streets? Before you can even find it, it probably already ends up in someone
else stomach. (I hate eating meat but this is just an phrase)

Saying that, I just wanted to say SIMD is not a silver bullet to everything.

We could also try employ the anti bound checking trick here as suggested by
@Licenser.

```rust
pub fn trick_simd_c16(s: &str) -> u64 {
    let d: &mut [u8; 16] = &mut b"0000000000000000".clone();
    let b: &[u8] = s.as_bytes();
    d.copy_from_slice(b);

    unsafe {
        let chunk = _mm_lddqu_si128(std::mem::transmute_copy(&d));
        let zeros = _mm_set1_epi8(b'0' as i8);
        let chunk = _mm_sub_epi16(chunk, zeros);

        let mult = _mm_set_epi8(1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10);
        let chunk = _mm_maddubs_epi16(chunk, mult);

        let mult = _mm_set_epi16(1, 100, 1, 100, 1, 100, 1, 100);
        let chunk = _mm_madd_epi16(chunk, mult);

        let chunk = _mm_packus_epi32(chunk, chunk);
        let mult = _mm_set_epi16(0, 0, 0, 0, 1, 10000, 1, 10000);
        let chunk = _mm_madd_epi16(chunk, mult);

        let chunk = _mm_cvtsi128_si64(chunk) as u64;
        ((chunk & 0xffffffff) * 100000000) + (chunk >> 32)
    }
}
```

Sad, it is not much of a help (10ns).

```
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
test bench_unrolled_safe       ... bench:          15 ns/iter (+/- 1) = 1066 MB/s
test bench_trick               ... bench:           6 ns/iter (+/- 0) = 2666 MB/s
test bench_trick_simd          ... bench:           8 ns/iter (+/- 1) = 2000 MB/s
test bench_trick_simd_c16      ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
```

# The 128 bit trick

The one is not in the original article, since rust supports 128-bits integer,
why not try it out? Using this is simpler than the previous `trick` function
since we need not to break the 16 characters into 2 parts, just work it out as
a whole. Awesome (つごい！ - "tsugoi" Japanese way of expressing jaw dropping),
we can even do a 8-byte mask trick.

```rust
pub fn trick_128(s: &str) -> u64 {
    let s = s.as_ptr() as *const _;
    let mut chunk = 0_u128;
    unsafe {
        std::ptr::copy_nonoverlapping(s, &mut chunk, std::mem::size_of_val(&chunk));
    }

    // 1-byte mask trick (works on 8 pairs of single digits)
    let lower_digits = (chunk & 0x0f000f000f000f000f000f000f000f00) >> 8;
    let upper_digits = (chunk & 0x000f000f000f000f000f000f000f000f) * 10;
    let chunk = lower_digits + upper_digits;

    // 2-byte mask trick (works on 4 pairs of two digits)
    let lower_digits = (chunk & 0x00ff000000ff000000ff000000ff0000) >> 16;
    let upper_digits = (chunk & 0x000000ff000000ff000000ff000000ff) * 100;
    let chunk = lower_digits + upper_digits;

    // 4-byte mask trick (works on 2 pairs of four digits)
    let lower_digits = (chunk & 0x0000ffff000000000000ffff00000000) >> 32;
    let upper_digits = (chunk & 0x000000000000ffff000000000000ffff) * 10000;
    let chunk = lower_digits + upper_digits;

    // 8-byte mask trick (works on a pair of eight digits)
    let lower_digits = (chunk & 0x00000000ffffffff0000000000000000) >> 64;
    let upper_digits = (chunk & 0x000000000000000000000000ffffffff) * 100000000;
    let chunk = lower_digits + upper_digits;

    chunk as u64
}
```

Oh, this is even faster than `trick_simd` but it is slightly slower (1ms) slower
than `trick`. Still, this method is way easier than `trick` itself such that it
looks straightforward without divide and conquer.

```
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
test bench_unrolled_safe       ... bench:          15 ns/iter (+/- 1) = 1066 MB/s
test bench_trick               ... bench:           6 ns/iter (+/- 0) = 2666 MB/s
test bench_trick_128           ... bench:           7 ns/iter (+/- 0) = 2285 MB/s
test bench_trick_simd          ... bench:           8 ns/iter (+/- 1) = 2000 MB/s
```

# Summary

Compilers are absolutely amazing pieces of technology. They regularly surprise
the original author (or even blow the original author's mind) at how well they
can optimize code and see through what the original author is doing.

Any last words? I wish serde have support for reading fixed sized strings,
if not it would be good to have a crate that have support for it like num. I
have nothing to say here except `trick` being the fastest and mind-blown by
SIMD not being the fastest.

Having said all that, there is a culture of "optimization is the root of all
evil". That handwritten assembly or hand-optimization has no place anymore. That
we should just blindly rely on godbolting and hoping that `rustc` do well.

I think both positions are complementary - trust your compiler (friend), trust
your library vendor, but nothing beats carefully thought out code when you know
your inputs and you have done your measurements to know it will make a
difference.

用人不疑，疑人不用。 (English direct translate: Use someone don't distrust,
distrust someone don't use, indirect translate: trust everyone, easy :P)
If you do not trust the compiler, why use it?

Imagine your business revolved around parsing a firehose of telemetry data, and
you chose to use `str::parse`. Would you buy more servers or spend a little time
optimizing your parsing?

```
test bench_naive_bytes         ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_bytes_and     ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_naive_bytes_and_c16 ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test bench_naive_bytes_iter    ... bench:          11 ns/iter (+/- 0) = 1454 MB/s
test bench_naive_chars         ... bench:          18 ns/iter (+/- 0) = 888 MB/s
test bench_naive_chars_and     ... bench:          17 ns/iter (+/- 1) = 941 MB/s
test bench_naive_chars_iter    ... bench:          18 ns/iter (+/- 1) = 888 MB/s
test bench_str_parse           ... bench:          23 ns/iter (+/- 1) = 695 MB/s
test bench_trick               ... bench:           6 ns/iter (+/- 0) = 2666 MB/s
test bench_trick_128           ... bench:           7 ns/iter (+/- 0) = 2285 MB/s
test bench_trick_simd          ... bench:           8 ns/iter (+/- 1) = 2000 MB/s
test bench_trick_simd_c16      ... bench:          10 ns/iter (+/- 0) = 1600 MB/s
test bench_unrolled            ... bench:          16 ns/iter (+/- 0) = 1000 MB/s
test bench_unrolled_safe       ... bench:          15 ns/iter (+/- 1) = 1066 MB/s
test bench_unrolled_unsafe     ... bench:          17 ns/iter (+/- 0) = 941 MB/s
```

# Post Scriptum

- All benchmarks ran on a 2.70 GHz Intel(R) Core(TM) i7-2620M, on Linux,
  compiled with rustc 1.46.0-nightly (0cd7ff7dd 2020-07-04).
- I did not do comparison with the original code.
- I did not get to try `parse_8_chars_simd` because `_mm_loadu_si64` is not
  available in `stdarch` ([pull request](https://github.com/rust-lang/stdarch/pull/870)).
- You can find the [code and the benchmarks here](https://github.com/pickfire/parseint).
