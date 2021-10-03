#[cfg(test)]
pub mod test {
    use std::fs;
    use std::time::{Duration, Instant};

    use rand::Rng;
    use rand::distributions::Alphanumeric;

    pub fn with_test_dir<F: FnOnce(&str) -> ()>(func: F) {
        let random_suffx: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let test_dir = format!("/tmp/rust-testing-{}", random_suffx);

        fs::create_dir_all(test_dir.clone()).expect("Unable to create test dir");

        func(&test_dir);

        fs::remove_dir_all(test_dir).expect("Unable to remote test dir");
    }

    pub fn time_func<F: FnOnce() -> ()>(func: F) -> Duration {
        let start = Instant::now();
        func();
        let end = Instant::now();
        return end - start;
    }
}
