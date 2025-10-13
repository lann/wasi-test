use wasi_test::TestContext;

fn test_pass(ctx: &TestContext) {
    ctx.log("Passing test logs typically aren't printed by default");
}

fn test_fail(ctx: &TestContext) -> Result<(), &'static str> {
    ctx.log("This test is doomed");
    Err("an expected failure is still a failure")
}

wasi_test::suite!(test_pass, test_fail);
