#![allow(dead_code)]
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[ts(export_to = "/tmp/ts_rs_test_type_a.ts")]
pub struct TestTypeA<T> {
    value: T,
}

#[derive(TS)]
#[ts(export)]
#[ts(export_to = "/tmp/ts_rs_test_type_b.ts")]
pub struct TestTypeB<T> {
    value: T,
}

#[derive(TS)]
#[ts(export_to = "/tmp/ts_rs_test_enum.ts")]
pub enum TestEnum {
    C { value: TestTypeB<i8> },
    A1 { value: TestTypeA<i32> },
    A2 { value: TestTypeA<i8> },
}

#[test]
#[cfg(feature = "format")]
fn test_def() {
    // The only way to get access to how the imports look is to export the type and load the exported file
    TestEnum::export().unwrap();
    let text = std::fs::read_to_string(TestEnum::EXPORT_TO.unwrap()).unwrap();

    // Checks to make sure imports are ordered and deduplicated
    assert_eq!(text,
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n",
            "import type { TestTypeA } from \"./ts_rs_test_type_a\";\n",
            "import type { TestTypeB } from \"./ts_rs_test_type_b\";\n",
            "\n",
            "export type TestEnum = { C: { value: TestTypeB<number> } } | {\n",
            "  A1: { value: TestTypeA<number> };\n",
            "} | { A2: { value: TestTypeA<number> } };\n"
        )
    );

    std::fs::remove_file(TestEnum::EXPORT_TO.unwrap()).unwrap();
}
