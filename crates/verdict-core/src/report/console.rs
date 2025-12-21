use crate::model::{TestResultRow, TestStatus};

pub fn print_summary(results: &[TestResultRow]) {
    let mut pass = 0;
    let mut fail = 0;
    let mut flaky = 0;
    let mut warn = 0;
    let mut error = 0;

    for r in results {
        match r.status {
            TestStatus::Pass => pass += 1,
            TestStatus::Fail => {
                fail += 1;
                eprintln!("FAIL [{}]: {}", r.test_id, r.message);
            }
            TestStatus::Flaky => {
                flaky += 1;
                eprintln!("FLAKY [{}]: {}", r.test_id, r.message);
            }
            TestStatus::Warn => {
                warn += 1;
                eprintln!("WARN [{}]: {}", r.test_id, r.message);
            }
            TestStatus::Error => {
                error += 1;
                eprintln!("ERROR [{}]: {}", r.test_id, r.message);
            }
        }
    }

    eprintln!(
        "Results: pass={} fail={} flaky={} warn={} error={}",
        pass, fail, flaky, warn, error
    );
}
