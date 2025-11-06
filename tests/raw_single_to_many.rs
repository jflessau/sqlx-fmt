mod common;

#[test_log::test]
fn raw_single_to_many() {
    let content = r###"
    sqlx::migrate!(
        r#"select *     from test where id = '1' and on = true and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar'"#,
    )
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff", 4, &None).unwrap();

    let expected = r###"
    sqlx::migrate!(
        r#"
            select *
            from test
            where
                id = '1'
            and on = true and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar'
        "#,
    )
    "###;

    common::compare(expected, &formatted);
}
