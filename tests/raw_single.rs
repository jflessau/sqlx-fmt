mod common;

#[test_log::test]
fn raw_single() {
    let content = r###"
    sqlx::migrate!(
        r#"alter table    test add column log text"#,
    )
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff", 4, &None).unwrap();

    let expected = r###"
    sqlx::migrate!(
        r#"alter table test add column log text"#,
    )
    "###;

    common::compare(expected, &formatted);
}
