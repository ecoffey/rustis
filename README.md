rustis
======

Writing a Redis client in Rust, for the learnings

building
========

Currently using rust-head (`brew install rust --HEAD`) for development, and rustpkg.

To build:

```
rustpkg build rustis
```

testing
=======

The test file expects to see a localhost redis running on the default port.

Given that a:

```
rustpkg clean rustis && rustpkg test rustis
```

should show the one (and only :-)) ping test passing

