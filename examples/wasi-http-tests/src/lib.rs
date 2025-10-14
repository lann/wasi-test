fn test_standard_header(
    ctx: &wasi_test::TestContext,
) -> Result<(), wasi::http::types::HeaderError> {
    ctx.log("This runner doesn't print logs for passing tests");
    wasi::http::types::Headers::from_list(&[("authorization".to_string(), vec![])])?;
    Ok(())
}

fn test_exploding_header(
    ctx: &wasi_test::TestContext,
) -> Result<(), wasi::http::types::HeaderError> {
    const NAME: &str = "🤯";
    ctx.log(format!("Trying header name {NAME:?}"));
    wasi::http::types::Headers::from_list(&[("🤯".to_string(), vec![])])?;
    Ok(())
}

wasi_test::suite!(test_standard_header, test_exploding_header);
