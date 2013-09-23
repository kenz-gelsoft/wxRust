native.rs: wxHaskell/wxc/src/include/wxc_glue.h codegen.py
	python codegen.py $< > native.rs

