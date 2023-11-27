use ::mongodb::Database;

use ::rpc::exchanges::Exchanges;

use super::traits::SymbolReader;

pub async fn get_reader(
  db: &Database,
  exchange: Exchanges,
) -> Box<dyn SymbolReader + Send + Sync> {
  return match exchange {
    Exchanges::Binance => {
      Box::new(super::binance::recorder::SymbolWriter::new(&db).await)
    }
  };
}
