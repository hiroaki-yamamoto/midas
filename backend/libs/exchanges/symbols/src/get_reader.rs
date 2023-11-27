use ::mongodb::Database;

use ::rpc::exchanges::Exchanges;

use super::traits::SymbolReader;

pub async fn get_reader(
  db: &Database,
  exchange: Box<Exchanges>,
) -> Box<dyn SymbolReader + Send + Sync> {
  return match exchange.as_ref() {
    Exchanges::Binance => {
      Box::new(super::binance::recorder::SymbolWriter::new(&db).await)
    }
  };
}
