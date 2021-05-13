%title: PyOxidizer
%author: Ivan Tham
%date: 2021-05-12
%usage: mdp slide.md

-> PyOxidizer <-
================

Packaging and distributing python applications.

---

-> Use cases <-
================

- deploying single binary for python projects
- packaging python project with dependencies
- at the same time use rust integration stuff

---

-> Installation <-
==================

- python - `pip install pyoxidizer`
- rust - `cargo install pyoxidizer`
- package manager

---

-> First impression <-
======================

- Confusing usage and docs
- Two entrypoints? `init-config-file` vs `init-rust-project`?

---

-> Entrypoints <-
=================

- `init-config-file`
  - creates blank project with a config file 
  - defaults to python interpreter
- `init-rust-project`
  - it is really a rust project
  - ... invoking the python interpreter
  - Why? ¯\\_(ツ)_/¯
- compared to the others this is harder to setup but more features around
  python rust integration

---

```
 
         /^\    /^\
        {  O}  {  O}
         \ /    \ /
         //     //       _------_
        //     //     ./~        ~-_
       / ~----~/     /              \
     /         :   ./       _---_    ~-
    |  \________) :       /~     ~\   |
    |        /    |      |  :~~\  |   |
    |       |     |      |  \___-~    |
    |        \ __/`=1C______\.        ./
     \                     ~-______-~\.
     .|                                ~-_
    /_____________________________________~~____
 
```

---

-> Side tips <-
===============

When experimenting with pyoxidizer, you may want to use sccache to
improve the build time it you run it quite often.

How slow? ~40s -> ~30s (incremental)
Just set `RUSTC_WRAPPER=sccache`.

---

-> Conclusion <-
================

- not easy to use although there are documentation (even hello world)

---

-> Sources <-
=============

- https://pyoxidizer.readthedocs.io/en/stable/

