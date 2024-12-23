`just` is a handy way to save and run project-specific commands.

Commands are stored in a file called `justfile` or `Justfile` with syntax
inspired by `make`:

```make
build:
    cc *.c -o main

# test everything
test-all: build
    ./test --all

# run a specific test
test TEST: build
    ./test --test {{TEST}}
```

`just` produces detailed error messages and avoids `make`'s idiosyncrasies, so
debugging a justfile is easier and less surprising than debugging a makefile.

It works on all operating systems supported by Rust.

Read more on [GitHub](https://github.com/casey/just).
