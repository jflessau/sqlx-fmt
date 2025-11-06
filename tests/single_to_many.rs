mod common;

#[test_log::test]
fn single_to_many() {
    let content = r###"
        sqlx::query!("select *     from test where id = '1' and on = true and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar';")
    "###;

    let expected = r###"
        sqlx::query!("select * from test where id = '1' and on = true and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar' and foo = 'bar';")
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff", 4, &None).unwrap();
    common::compare(expected, &formatted);
}
