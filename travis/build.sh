cd rust-bindgen
patch -p1 < ../travis/rust-bindgen-workaround.patch
cd ..

mkdir -p build
cd build
cmake ..
make
