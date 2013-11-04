#[macro_escape];

pub macro_rules! wxApp(
    ($f: ident) => (
        fn start(argc: int, argv: **u8) -> int {
            #[start];
            fn on_main() {
                #[fixed_stack_segment];
                #[inline(never)];
                let closure = wxClosure::new($f as *mut c_void, nullptr);
                let args: ~[*i32] = ~[];
                ELJApp::initializeC(closure, args.len() as i32, vec::raw::to_ptr(args) as *mut *mut i8);
            }
            start_on_main_thread(argc, argv, on_main)
        }
    )
)
