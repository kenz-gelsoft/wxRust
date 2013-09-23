libwxrust.a: native.rs types.rs wx.rc
	rustc -L wxc wx.rc -o libwxrust.a

native.rs: wxHaskell/wxc/src/include/wxc_glue.h codegen.py
	python codegen.py $< > $@
	git diff $@	

