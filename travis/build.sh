cd rust-bindgen
patch -p1 < ../travis/rust-bindgen-workaround.patch
cd ..

mkdir -p build
cd build
cmake -DRUSTCFLAGS="-L;/usr/lib/llvm-3.3/lib" ..
make
