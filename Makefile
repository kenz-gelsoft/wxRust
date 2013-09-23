GENSRCDIR=wxHaskell/wxc/src/include/
GENSRCS = \
	$(GENSRCDIR)wxc_glue.h \
	$(GENSRCDIR)wxc.h \


libwx.dummy: native.rs types.rs wx.rc
	rustc -L wxc wx.rc
	touch libwx.dummy

native.rs: $(GENSRCS) codegen.py
	python codegen.py $(GENSRCS) > $@
	git diff $@	

test: test.rs libwx.dummy
	rustc -L wxc -L . test.rs

clean:
	rm libwx.dummy
	rm test
