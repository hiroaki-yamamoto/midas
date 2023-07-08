use ::config::init;

#[tokio::main]
async fn main() {
  init(|cfg, sig, db, broker, _| async {}).await;
}
