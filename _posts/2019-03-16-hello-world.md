---
layout: post
title:  "Hello world!"
date:   2019-03-11 09:09:29 +0800
categories: community
---

Rust Malaysia has hosted a few meetups in Kuala Lumpur in the past few months.
We are now planning to host it 

> Rust is a systems language pursuing the trifecta: safety, concurrency, and
> speed.

Dear Malaysians, welcome to Rust Programming Language! To get started, use
[rustup.rs][rustup] which streamlines the rust installation processes.

{% highlight shell %}
$ curl https://sh.rustup.rs -sSf | sh
{% endhighlight %}

Bootstrap a new rust project (or just use [rust playground][playground]).

{% highlight rust %}
$ cargo new hello
$ cd hello
{% endhighlight %}

Edit the rust source files in `src/main.rs`, need not to worry about the `!`.
Due to the diversity of cultures in Malaysia, we will use multiple languages.

{% highlight rust %}
fn main() {
    // note that this is sorted by length in terminal
    let greetings = [
        "ஹலோ, மலேசியா!"
        "你好，马来西亚！",
        "Hello, Malaysia!",
        "Apa khabar, Malaysia!",
    ];

    for greeting in greetings.iter() {
        println!("{}", greeting);
    }
}
{% endhighlight %}

Build and profit!

{% highlight shell %}
$ cargo run
{% endhighlight %}

For more resources on rust, check out the [official rust docs][rust-docs].

We will probably publish resources discussed during rust meetups in Malaysia.
Stay tuned for more posts! :)

[rustup]:     https://rustup.rs
[rust-docs]:  https://doc.rust-lang.org
[playground]: https://play.rust-lang.org
