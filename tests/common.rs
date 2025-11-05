use text_diff::diff;

pub fn compare(expected: &str, formatted: &str) {
    let expected = expected
        .replace("\\", "\\\\")
        .replace("\t", "\\t")
        .replace("\n", "\\n");
    let formatted = formatted
        .replace("\\", "\\\\")
        .replace("\t", "\\t")
        .replace("\n", "\\n");

    if expected != formatted {
        println!("expected:\n:{expected}");
        println!("formatted:\n:{formatted}");
        panic!(
            "formatted content does not match expected: \n{:?}",
            diff(&expected, &formatted, "")
        );
    }
}
