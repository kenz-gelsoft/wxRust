libwxrust.a: native.rs
	rustc native.rs -o libwxrust.a

native.rs: wxHaskell/wxc/src/include/wxc_glue.h codegen.py
	python codegen.py $< > $@
	git diff $@	

