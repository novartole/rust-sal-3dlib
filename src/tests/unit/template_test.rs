#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing such functionality / behavior
    #[test]
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg_id = DbgId::new("test");
        log::debug!("\n{}", dbg_id);
        let test_duration = TestDuration::new(dbg_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        assert!(result == target, "{}.intersection | step {} \nresult: {:?}\ntarget: {:?}", dbg_id, step, result, target);
        test_duration.exit();
    }
}