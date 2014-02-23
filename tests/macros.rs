#[macro_escape];

pub macro_rules! wxApp(
    ($f: ident) => (
        fn start(argc: int, argv: **u8) -> int {
            #[start];

            use std::libc::c_void;
            use std::rt::start_on_main_thread;
            use std::vec;

            use wx::base::WxClosure;
            use wx::core::WxrApp;

            static nullptr: *mut c_void = 0 as *mut c_void;

            fn on_main() {
                #[fixed_stack_segment];
                #[inline(never)];
                let closure = WxClosure::new($f as *mut c_void, nullptr);
                let args: ~[*i32] = ~[];
                WxrApp::initializeC(&closure, args.len() as i32, vec::raw::to_ptr(args) as *mut *mut i8);
            }
            start_on_main_thread(argc, argv, on_main)
        }
    )
)
