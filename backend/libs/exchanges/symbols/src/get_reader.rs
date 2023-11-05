use ::mongodb::Database;

use ::rpc::exchange::Exchange;

use super::traits::SymbolReader;

pub async fn get_reader(
  db: &Database,
  exchange: Exchange,
) -> Box<dyn SymbolReader + Send + Sync> {
  return match exchange {
    Exchange::Binance => {
      Box::new(super::binance::recorder::SymbolWriter::new(&db).await)
    }
  };
}
