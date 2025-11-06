use text_diff::diff;

pub fn compare(expected: &str, formatted: &str) {
    if expected != formatted {
        println!("expected vs. formatted\n{expected}\n\n---\n\n{formatted}");

        panic!(
            "formatted content does not match expected: \n{:#?}",
            diff(&expected, &formatted, "")
        );
    }
}
