#![macro_use]

macro_rules! wxApp(
    ($f: ident) => (
        #[start]
        fn start(argc: isize, argv: *const *const u8) -> isize {
            use libc::c_void;

            use lib::base::Closure;
            use lib::core::RustApp;

            const NULLPTR: *mut c_void = 0 as *mut c_void;

            let closure = Closure::new($f as *mut c_void, NULLPTR);
            let args: Vec<*mut i32> = Vec::new();
            RustApp::initializeC(&closure, args.len() as i32, args.as_ptr() as *mut *mut i8);
            0
        }
    )
);
