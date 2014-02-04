#[macro_escape];

pub macro_rules! wxApp(
    ($f: ident) => (
        fn start(argc: int, argv: **u8) -> int {
            #[start];

            use std::libc::c_void;
            use std::vec;

            use wx::base::wxClosure;
            use wx::core::ELJApp;

            static nullptr: *mut c_void = 0 as *mut c_void;

            native::start(argc, argv, proc() {
                let closure = wxClosure::new($f as *mut c_void, nullptr);
                let args: ~[*i32] = ~[];
                ELJApp::initializeC(closure, args.len() as i32, args.as_ptr() as *mut *mut i8);
            })
        }
    )
)
