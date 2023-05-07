# WolfGrep
Why? I don't know... Learning

---
# Introduction

I was heavily curious to how grep works and if I could rpelicate it.
Which I did.

I still have some plans for future expansion to make it faster for larger files.

# Build

```bash
$ cargo build --release
```

# Tests?

I am sorry Rory, >.<

# Currently

It runs faster than systems grep by couple microseconds :)

Grep:
```bash
$ time grep "matches" src/main.rs
...
Executed in  872.00 micros    fish           external
   usr time  631.00 micros    0.00 micros  631.00 micros
   sys time  299.00 micros  299.00 micros    0.00 micros
```

WGrep:
```bash
$ time wgrep "matches" src/main.rs
...
Executed in  781.00 micros    fish           external
   usr time  644.00 micros    0.00 micros  644.00 micros
   sys time  183.00 micros  183.00 micros    0.00 micros
```