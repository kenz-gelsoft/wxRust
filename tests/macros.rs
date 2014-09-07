#![macro_escape]

pub macro_rules! wxApp(
    ($f: ident) => (
        #[start]
        fn start(argc: int, argv: **u8) -> int {

            use libc::c_void;

            use wx::base::Closure;
            use wx::core::RustApp;

            static nullptr: *mut c_void = 0 as *mut c_void;

            native::start(argc, argv, proc() {
                let closure = Closure::new($f as *mut c_void, nullptr);
                let args: Vec<*i32> = Vec::new();
                RustApp::initializeC(&closure, args.len() as i32, args.as_ptr() as *mut *mut i8);
            })
        }
    )
)
