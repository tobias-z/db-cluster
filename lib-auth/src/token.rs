use rand::{distributions::Alphanumeric, Rng};

pub fn generate_join_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(70)
        .map(char::from)
        .collect()
}
