# so-41492672-rust-js-structure
[How can you compile a Rust library to target asm.js?](http://stackoverflow.com/questions/41492672/how-can-you-compile-a-rust-library-to-target-asm-js) aka Rust JavaScript big application structure

Very interesting question! I was running into [similar dependency issues](https://github.com/fable-compiler/Fable/issues/777) with [fable](http://fable.io/).

I have checked [Compiling Rust to your Browser - Call from JavaScript](http://www.hellorust.com/emscripten/demos/04-call-into-lib/), [Advanced Linking - Link args](https://doc.rust-lang.org/book/advanced-linking.html) and [How to pass cargo linker args](https://users.rust-lang.org/t/how-to-pass-cargo-linker-args/3163) however was not able to use `cargo` in the same way as [`rustc --target asmjs-unknown-emscripten call-into-lib.rs`](http://www.hellorust.com/emscripten/demos/04-call-into-lib/).

The closer I was able to get was to run both `cargo` and `rustc` like

    cd lib1
    cargo build --target asmjs-unknown-emscripten
    rustc --target=asmjs-unknown-emscripten src\lib.rs
    cd ..
    
    cd lib2
    cargo build --target asmjs-unknown-emscripten
    rustc --target=asmjs-unknown-emscripten src\lib.rs --extern lib1=..\lib1\target\asmjs-unknown-emscripten\debug\liblib1.rlib
    cd ..
    
    cd lib3
    cargo build --target asmjs-unknown-emscripten
    rem rustc --target=asmjs-unknown-emscripten src\lib.rs --extern webplatform=..\lib3\target\asmjs-unknown-emscripten\debug\deps\libwebplatform-80d107ece17b262d.rlib
    rem the line above fails with "error[E0460]: found possibly newer version of crate `libc` which `webplatform` depends on"
    cd ..
    
    cd app
    cargo build --target asmjs-unknown-emscripten
    cd ..

see the [so-41492672-rust-js-structure](https://github.com/davidpodhola/so-41492672-rust-js-structure/blob/master/build.cmd)

I still think some manual linking would help. Would be interested to know.
