use unite::unite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Foo(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bar(bool);

unite! {
    /// Test enum.
    enum Test {
        Foo,
        Baz = Bar,
        Void = (),
    }
}

const FOO: Test = Test::Foo(Foo(0));
const BAZ: Test = Test::Baz(Bar(true));
const VOID: Test = Test::Void(());

#[test]
fn checks() {
    assert_eq!(FOO.is_foo(), true);
    assert_eq!(FOO.is_baz(), false);
    assert_eq!(FOO.is_baz(), false);

    assert_eq!(BAZ.is_foo(), false);
    assert_eq!(BAZ.is_baz(), true);
    assert_eq!(BAZ.is_void(), false);

    assert_eq!(VOID.is_foo(), false);
    assert_eq!(VOID.is_baz(), false);
    assert_eq!(VOID.is_void(), true);
}

#[test]
fn casting() {
    assert_eq!(FOO.as_foo(), Some(&Foo(0)));
    assert_eq!(FOO.as_baz(), None);
    assert_eq!(FOO.as_void(), None);

    assert_eq!(BAZ.as_foo(), None);
    assert_eq!(BAZ.as_baz(), Some(&Bar(true)));
    assert_eq!(BAZ.as_void(), None);

    assert_eq!(VOID.as_foo(), None);
    assert_eq!(VOID.as_baz(), None);
    assert_eq!(VOID.as_void(), Some(&()));
}
