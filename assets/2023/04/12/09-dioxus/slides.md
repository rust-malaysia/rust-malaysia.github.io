---
---

# From react to dioxus

---
---

## React

- most popular javascript framework
- uses VDOM
- targets web by default
  - react-native targets mobile
  - electron targets desktop

---
---

## Dioxus

- one of the rust frontend framework aiming to be ergonomic
- based on react (hooks)
- uses tauri under the hood
- uses VDOM (although I hope it does not)
- targets multiple platforms under the same organization (but still unstable)

---
---

## Platform support

- native (good)
- web (best)
  - **slow reload** (25s for word change) enabled by default (probably first, not even yew or sycamore)
  - **hot reload** only works when specified `--hot-reload`, instant output
- SSR
  - dioxus virtualdom `!Send` so
    - cannot hold virtualdom across `.await` points
    - which is required by most web frameworks like axum
- liveview
  - first time I hear something like this
  - run app in server and render in browser
- mobile (poor)
- terminal (experimental)
  - renders with html (some css) instead of custom markup

---
---

![hot reload](/dioxus-web-hot-reload.png)

---
---

## About the talk

- mainly on frontend web with dioxus
- note, sycamore was previously discussed in a past meetup
- why dioxus? (instead of yew, seed, sycamore, egui, iced, http://www.areweguiyet.com/)
  - https://blog.logrocket.com/current-state-rust-web-frameworks/
  - cross-platform (mobile support), ergonomic, hot-reloading
  - innovative (reminds me of bevy) at a fast pace
- want to see what is the state of dioxus (based on experiments)
  - does it have foot guns? tl;dr yes
  - if it compiles, it works? tl;dr no (mainly due to crates wasm support)
  - interesting new ideas but not there yet
- did two experiments
  - improving todomvc
  - try out a react foot gun as mentioned in website

---
---

# Improving todomvc on dioxus-desktop examples

https://github.com/DioxusLabs/dioxus/pull/928

Changes

- toggle-all button
- double click modify todo item
- filter state show mouse pointer
- individual todo item remove button
- correct active item count

---
---

## Good (1)

- function-based, simple and straightforward
  ```rust
  fn main() {
      dioxus_web::launch(app);
  }

  fn app(cx: Scope) -> Element {
      cx.render(rsx! {
          div {
              "Hello, world!"
          }
      })
  }
  ```
- ergonomic (compared to most other rust alternatives)
  - no elm-style messaging (like in seed)
  - greppable and simple hooks - `let count = use_state(cx, || 0);`

---
---

## Good (2)

- hot-reloading
  - very fast (~1s) but no HMR if configured correctly
- home-built `rsx!` (like `jsx`)
  - works even with rust for loops, if, if-let (to my surprise)
  - format args support with caveat (not usable with conditional)
  - no html closing tag, E.g. `</div>`
  ```rust
  rsx! {
      div {
          class: "bg-dark",
          onclick: |_event| { /* do something */ },
          h1 { "Header" }
          "Hello, {x}!"
      }
  }
  ```
  - works even with `match`! but `rsx!` integration not that good
    (have to use `rsx!` within match body)

---
---

## Bad (1)

- boolean attributes need to pass in `"true"` and `"false"`
  ```rust
  input {
      checked: if active_todo_count == 0 { "true" } else { "false" }
  }
  ```
- `UseState` no compile-time lifetime check (runtime instead)
  - so it panics at runtime if borrow when there is a mutable borrow
  - not a big deal as one can reduce `make_mut` to minimum event handling
  - so far no issue unless I purposely put the `make_mut` in main block
- `ondoubleclick` is `ondblclick`? I was wondering why it didn't
  ```rust
  label {
      r#for: "cbg-{todo.id}",
      ondblclick: move |_| is_editing.set(true),
  }
  ```
- docs suggest non-standard casing `#![allow(non_snake_case)]`

---
---

## Bad (2)

- some stuff like configuring css styles is not clear and consistent (across platforms)
  - desktop
    ```rust
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()),
    );
    ```
  - web need to use `jsx!` and took me some time to figure this
    ```toml
    # Dioxus.toml
    [web.resource]
    style = [
      "https://cdn.jsdelivr.net/npm/daisyui@2.51.5/dist/full.css",
    ]
    script = [
      "https://cdn.tailwindcss.com",
    ]
    ```

---
---

## Bad (3)

- there are type-safe css class plugins (tailwindcss/daisyui)
  ```rust
  cx.render(rsx!{
      div {
          class: class!(card_body text_center items_center hover(scale_105)),
          div {
              class: class!(card_title text_sm text_base_content),
              cx.props.alias
          }
      }
  })
  ```
  - but no proprocessor so file size will be large (have to use full sized css)
  - requires full code compilation which makes hot-reload useless
- most libraries does not work well with wasm - runtime error or unexpected behavior
  - I experienced breakage related to rand (lorem ipsum)
  - I thought it is dioxus issue that all lorem ipsum text is the same
  - turns out lorem ipsum was not working correctly for wasm (due to rand?)
  - lesson is that probably more crates than you imagined does not work

---
---

## Bad (4)

- no official documentation on how to mutate borrowed props, at the end have to
  refer to some examples with some tweak
  ```rust
  fn app(cx: Scope) -> Element {
      let nth = use_state(cx, || 0);
      rsx! {
          label_item { nth: nth }
      }
      ...
  }
  #[derive(Props)]
  struct LabelItemProps<'a> {
      nth: &'a UseState<usize>,
  }
  fn label_item<'a>(cx: Scope<'a, LabelItemProps<'a>>) -> Element {
      cx.render(rsx! {
          button {
              class: "btn",
              onclick: move |_| *cx.props.nth.make_mut() += 1,
              "Emission 1"
          }
      }
  }
  ```

---
---

## Ugly (1)

- errors can be confusing at times
  ```rust
  for choice in history {
      p { "{choice:?}" }
  }
  ```
  ```rust
  \ Compiling dioxus-web 0.3.1 (registry+https://github.com/rust-lang/crates.io-inerror: could not compile `dioxus-demo` due to 2 previous errors
  [ERROR] error: expected `,`
     --> src/main.rs:118:31
      |
  118 | ...                   p { "{choice}" }
      |                         ^
  ```
  ```rust
  for choice in &history.read() { // correct
      p { "{choice:?}" }
  }
  ```

---
---

## Learning experience (on todomvc)

- took ~2.5 hours to learn and change a broken todomvc
  - https://github.com/DioxusLabs/dioxus/pull/928
- only read some pages in docs, not even all of it and start changing
- seemed pretty straightforward, compiler-driven development works
- just one function to render and do everything, simple but a bit weird
- no footguns encountered so far, but the variables work like magic
  ```rust
  let todos = use_state(cx, im_rc::HashMap::<u32, TodoItem>::default);
  let active_todo_count = todos.values().filter(|item| !item.checked).count();
  ...
  rsx! {
      span { class: "todo-count",
          strong {"{active_todo_count} "}
          span {"{active_todo_text} left"}
      }
  }
  ```
- 4 spaces nesting level seemed a bit too much compared to js
- I find it easier than react, just change random code, check compiler to fix

---
---

## Hello world (react)

1. `yarn create react-app hello` (template)
2. `yarn start`
3. Edit `src/App.js`

```js
export default function App() {
  return (
    <div>
      Hello, world!
    </div>
  );
}
```

---
---

## Hello world (dioxus web)

1. `cargo new hello`
2. `cargo add dioxus dioxus-web`
3. `dioxus serve --hot-reload` (requires `cargo install dioxus-cli`)
4. Edit `src/main.rs`

```rust
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
```

To support different platform, change `dioxus_web` to `dioxus_desktop`.

---
---

## Hello world (dioxus web) caveat

If I add

```rust
    ...
    let a = 1;  // <- this
    cx.render(rsx! {
    ...
```

Default `rust-analyzer` and `dioxus-cli` results in ~**27s** rebuild.

A fix is to add to `.cargo/config` (not documented in docs):

```toml
[build]
target = "wasm32-unknown-unknown"
```

Rebuild time now ~**1s**.

---
---

## Dioxus CLI

- looks ugly on stable version ![dioxus cli output](/dioxus-cli-output.png)

---
---

## Trying out some react footguns

https://jakelazaroff.com/words/were-react-hooks-a-mistake/

Let's see if dioxus inherit react footguns, even though react say they did.

```js
function CounterButton({ started, count, onClick }) {
  return <button onClick={onClick}>{started ? "Current score: " + count : "Start"}</button>;
}

...  // a bit long, see next page
```

---
---

```js
class Game extends React.Component {
  state = { count: 0, started: false };

  increment() {
    this.setState({ count: this.state.count + 1 });
  }

  start() {
    if (!this.state.started) setTimeout(() => alert(`Your score was ${this.state.count}!`), 5000);
    this.setState({ started: true });
  }

  render() {
    return (
      <CounterButton
        started={this.state.started}
        count={this.state.count}
        onClick={() => {
          this.increment();
          this.start();
        }}
      />
    );
  }
}
```

---
---

## React hooks with bug

```js
  const [count, setCount] = useState(0);
  const [started, setStarted] = useState(false);

  function increment() {
    setCount(count + 1);
  }

  function start() {
    if (!started) setTimeout(() => alert(`Your score was ${count}!`), 5000);
    setStarted(true);
  }

  return (
    <button
      onClick={() => {
        increment();
        start();
      }}
    >
      {started ? "Current score: " + count : "Start"}
    </button>
  );
```

---
---

## Bug-per-bug copy in dioxus

```rust
    let count = use_state(cx, || 0);
    let started = use_state(cx, || false);

    let start = || {
        cx.spawn({
            let has_started = started.to_owned();
            let count = count.to_owned(); // this is obvious that variable will not change
            started.set(true);
            async move {
                if !has_started {
                    let alert = move || gloo_dialogs::alert(&format!("Your score was {count}!"));
                    gloo_timers::callback::Interval::new(5_000, alert).forget();
                }
            }
        });
    };

    cx.render(rsx! {
        button {
            onclick: move |_event| {
                *count.make_mut() += 1;
                start();
            },
            // format is needed as {count} does not seemed to work in `if` within content
            if **started { format!("Current score: {count}") } else { "Start".to_string() }
        }
    })
```

---
---

## Bug-per-bug copy in dioxus

Working backword from correct solution. Need to change mental model.

```rust
    let count = use_state(cx, || 0);
    let started = use_state(cx, || false);

    let start = || {
        if !*started.get() {
            let count = count.clone(); // this is obvious that variable will not change
            let alert = move || gloo_dialogs::alert(&format!("Your score was {count}!"));
            gloo_timers::callback::Timeout::new(5_000, alert).forget();
        }
        started.set(true);
    };

    cx.render(rsx! {
        button {
            onclick: move |_event| {
                start();
                *count.make_mut() += 1;
            },
            // format is needed as {count} does not seemed to work in `if` within content
            if **started { format!("Current score: {}", count) } else { "Start".to_string() }
        }
    })
```

---
---

## React fixed

```js
  const [count, setCount] = useState(0);
  const [started, setStarted] = useState(false);
  const countRef = useRef(count);

  function increment() {
    setCount(count + 1);
    countRef.current = count + 1;
  }

  function start() {
    if (!started) setTimeout(() => alert(`Your score was ${countRef.current}!`), 5000);
    setStarted(true);
  }

  return (
    <button
      onClick={() => {
        increment();
        start();
      }}
    >
      {started ? "Current score: " + count : "Start"}
    </button>
  );
```

---
---

## Dioxus fixed

```rust
    let count = use_ref(cx, || 0); // use_ref
    let started = use_state(cx, || false);

    let start = || {
        if !*started.get() {
            let count = count.clone(); // clone reference rather than value
            let alert = move || gloo_dialogs::alert(&format!("Your score was {}!", count.read()));
            gloo_timers::callback::Timeout::new(5_000, alert).forget();
        }
        started.set(true);
    };

    cx.render(rsx! {
        button {
            onclick: move |_event| {
                start();
                *count.write() += 1;
            },
            // format is needed as {count} does not seemed to work in `if` within content
            if **started { format!("Current score: {}", count.read()) } else { "Start".to_string() }
        }
    })
```

Looks cleaner but took me some time to figure out too.

---
---

## Tokio caveat

No compile time error for tokio even when it does not support wasm.

In browser devtools,

```
panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:13:9
```

Can replace time with `gloo`, but probably might not work on dioxus-desktop?

https://gloo-rs.web.app/

---
---

## Take away with event count experiment

- dioxus examples and docs are limited, have to take some time to figure out
- not be able to port react to dioxus by line, have to change design
- more explicit code (E.g. `clone`) due to lifetime
- **footgun** when variable is set, function will rerun, infinite loop
  ```rust
    let start = || {
        if !*started.get() {
            started.set(true); // this cause infinite loop
            let count = count.clone(); // clone reference rather than value
            let alert = move || gloo_dialogs::alert(&format!("Your score was {}!", count.read()));
            gloo_timers::callback::Timeout::new(5_000, alert).forget();
        }
        // started.set(true); // this cannot be done inside condition or infinite loop
    };
  ```

---
---

## What's missing (WIP)

- not fully mature, still quite new
- type-checked css with pipeline
- image pipeline
- better cross-platform support
- better ecosystem support for wasm
- native rendering
