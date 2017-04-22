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
