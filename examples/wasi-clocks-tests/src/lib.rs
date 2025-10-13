use wasi_test::TestContext;
use wasip3::wit_bindgen;

fn test_clocks(_ctx: &TestContext) -> anyhow::Result<()> {
    let t1 = wasip3::clocks::monotonic_clock::now();
    wit_bindgen::yield_blocking();
    let t2 = wasip3::clocks::monotonic_clock::now();
    anyhow::ensure!(t2 >= t1);
    Ok(())
}

wasi_test::suite!(test_clocks);
