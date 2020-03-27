use unwind_aborts::unwind_aborts;

#[unwind_aborts]
fn foo() {}

#[test]
fn it_compiles() {
    foo();
}
