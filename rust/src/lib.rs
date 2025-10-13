use std::{
    borrow::Cow,
    fmt::Display,
    sync::{Arc, Mutex},
};

pub use wit_bindgen;

wit_bindgen::generate!({
    world: "suite",
    path: "../wit",
    generate_all,
    pub_export_macro: true,
});

use exports::wasi::test::tests;

pub use tests::TestResult;
use wit_bindgen::{StreamReader, StreamWriter};

// pub type TestFn =
//     Box<dyn for<'a> Fn(&'a TestContext) -> Pin<Box<dyn Future<Output = TestResult> + 'a>>>;
pub type TestFn = Box<dyn Fn(&TestContext) -> TestResult>;

pub struct TestCase {
    name: Cow<'static, str>,
    test_fn: TestFn,
}

impl TestCase {
    pub fn new(name: impl Into<Cow<'static, str>>, test_fn: TestFn) -> Self {
        Self {
            name: name.into(),
            test_fn,
        }
    }

    pub fn into_resource(self) -> tests::TestCase {
        tests::TestCase::new(self)
    }
}

impl tests::GuestTestCase for TestCase {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn run(&self, ctx: tests::TestContextBorrow<'_>) -> TestResult {
        (self.test_fn)(ctx.get())
    }
}

#[derive(Default)]
pub struct TestContext {
    log_writer: Arc<Mutex<Option<StreamWriter<String>>>>,
}

impl TestContext {
    #[allow(clippy::await_holding_lock)]
    pub fn log(&self, msg: impl Into<String>) {
        let msg = msg.into();
        let writer = self.log_writer.clone();
        wit_bindgen::block_on(async move {
            if let Some(w) = writer.lock().unwrap().as_mut() {
                w.write_one(msg).await;
            }
        })
    }
}

impl tests::GuestTestOptions for TestContext {
    fn new() -> Self {
        Default::default()
    }

    fn enable_logging(&self) -> StreamReader<String> {
        let (tx, rx) = wit_stream::new();
        *self.log_writer.lock().unwrap() = Some(tx);
        rx
    }
}

impl tests::GuestTestContext for TestContext {
    fn new(opts: tests::TestOptions) -> Self {
        opts.into_inner()
    }

    fn logging_enabled(&self) -> bool {
        self.log_writer.lock().unwrap().is_some()
    }

    fn log(&self, msg: String) {
        self.log(msg)
    }
}

pub trait IntoTestResult {
    fn into_test_result(self) -> TestResult;
}

impl IntoTestResult for () {
    fn into_test_result(self) -> TestResult {
        TestResult::Pass
    }
}

impl IntoTestResult for TestResult {
    fn into_test_result(self) -> TestResult {
        self
    }
}

impl<T: IntoTestResult, E: Display> IntoTestResult for Result<T, E> {
    fn into_test_result(self) -> TestResult {
        match self {
            Ok(ok) => ok.into_test_result(),
            Err(err) => TestResult::Fail(err.to_string()),
        }
    }
}

#[macro_export]
macro_rules! suite {
    (
        $( $fn:ident ),+ $(,)?
    ) => {
        mod _guest_impl {
            use $crate::{
                IntoTestResult as _,
                TestCase,
                TestContext,
                exports::wasi::test::tests,
                wit_bindgen,
            };

            struct Guest;

            impl tests::Guest for Guest {
                type TestCase = TestCase;
                type TestOptions = TestContext;
                type TestContext = TestContext;

                fn all() -> Vec<tests::TestCase> {
                    vec![
                        $({
                            let test_name = stringify!($fn).replace('_', "-");
                            let test_fn: $crate::TestFn = Box::new(|opts|
                                super::$fn(opts).into_test_result()
                            );
                            TestCase::new(test_name, test_fn).into_resource()
                        },)*
                    ]
                }
            }

            $crate::export!(Guest with_types_in $crate);
        }
    }
}
