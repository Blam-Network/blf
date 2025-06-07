#[macro_export]
macro_rules! BINRW_ERROR {
    ($task:expr) => {
        $task.map_err(|e| binrw::error::Error::Custom {
            pos: u64::MAX,
            err: Box::new(e.to_string()),
        })
    };
}