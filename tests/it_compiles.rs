use unwind_aborts::unwind_aborts;

#[unwind_aborts]
fn two() -> usize {
    2
}

#[test]
fn it_compiles() {
    assert_eq!(two(), 2);
}
