# Linker failure using the `extern "Rust" {}` magic

Project dep tree is as follows (`cargo tree):
```
mybinary v0.1.0 (/linker-failure/mybinary)
├── critical-section v1.1.1
├── mycriticalimpl v0.1.0 (/linker-failure/mycriticalimpl)
│   └── critical-section v1.1.1
└── mylib v0.1.0 (/linker-failure/mylib)
    └── critical-section v1.1.1
```

## Problem

UPDATE: Analysis below most likely incorrect. There is something ESP-IDF specific in the linking process.

The above dep-tree (I believe) is linearized in the following library order by the linker:
```
[mybinary], [mycriticalimpl], [mylib], [critical-section]
```

What is important in the above order is that `[mycriticalimpl]` is ordered *before* `[critical-section]`:
- This is so, because "smart linkers" like - say - GCC require to see the depending libnraries *before* their dependencies (see [this](https://stackoverflow.com/questions/45135/why-does-the-order-in-which-libraries-are-linked-sometimes-cause-errors-in-gcc) for more info)
- Since `[mycriticalimpl]` depends on `[critical-section]` by using symbols from it, it is ordered before `[critical-section]`. This way the linker can "record" what symbols
  `[mycriticalimpl]` needs and then can try to satisfy these from the subsequent libraries in the link order (`[critical-section]` specifically)
- The trouble is that the `extern "Rust" {}` magic introduces a *circular* dependency between `[mycriticalimpl]` and `[critical-section]` 
  (which is otherwise not allowed in Rust):
  - On one hand, `[mycriticalimpl]` (obviously) depends on `[critical-section]`, so the order should be `[mycriticalimpl], [critical-section]` 
    (and this is what Rust does, as it "sees" this dependency in the dependency graph)
  - On the other hand, `[critical-section]` *also* depends on `[mycriticalimpl]` via the magic `_critical_section_1_0_acquire` and `_critical_section_1_0_release` symbols!
  - However, Rust does not "know" about this dependency; and even if it did know, the order cannot be both `[mycriticalimpl], [critical-section]` and also `[critical-section], [mycriticalimpl]`...
  - (such circular dependencies are supported by linkers, but then these circular libraries need to be enclosed in `-lstart-group` / `-lend-group` switches); 

## Solution??

I don't have any besides instantiating the `critical_section::set_impl!(mycriticalimpl::cs::MyCriticalSection)` macro *from within the binary crate*. As noted on line 1 in `mybinary/main.rs`.

Why is that working? No idea (yet).
