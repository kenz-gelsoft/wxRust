cd rust-bindgen
patch -p1 < ../rust-bindgen-workaround.patch
cd ..

mkdir -p build
cd build
cmake ..
make
