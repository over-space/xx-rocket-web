pub fn dotenv() {
    dotenv::from_path(".env").ok();
}
