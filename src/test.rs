
fn run_test<T>(test: T) -> ()
    where T: FnOnce -> () + panic::UnwindSafe 
{
    setup();

    let result = panic::catch_unwind(||{
        test()
    });

    teardown();

    assert!(result.is_ok());
}

