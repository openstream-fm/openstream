use ts_rs::TS;

#[test]
fn free() {
    assert_eq!(<[String; 10]>::inline(), "Array<string>")
}

#[test]
fn newtype() {
    /// comment
    #[derive(TS)]
    struct Commented(String);

    assert!(Newtype::decl().contains("comment"))
}
