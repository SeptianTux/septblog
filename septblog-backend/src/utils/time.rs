pub fn current_unix_timestamp() -> u64 {
    let ret = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0
    };

    return ret;
}

pub fn time_ago() -> String {

    String::from("test")
}

