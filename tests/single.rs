mod common;

#[test_log::test]
fn single() {
    let content = r###"
        sqlx::query!("select *   from test where id = '1';")
    "###;

    let expected = r###"
        sqlx::query!("select * from test where id = '1';")
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff", 4, &None).unwrap();
    common::compare(expected, &formatted);
}
