use ::std::sync::Arc;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::future::{join, try_join_all, BoxFuture};
use ::futures::stream::BoxStream;
use ::futures::{FutureExt, StreamExt, TryFutureExt};
use ::log::{as_error, as_serde, error};
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::Database;
use ::rug::Float;

use ::entities::{
  BookTicker, ExecutionSummary, Order, OrderInner, OrderOption,
};
use ::errors::ExecutionResult;
use ::keychain::binance::APIKeySigner;
use ::keychain::{IKeyChain, KeyChain};
use ::observers::binance::TradeSubscriber;
use ::observers::traits::ITradeSubscriber as TradeSubscriberTrait;
use ::position::binance::{
  interfaces::IOrderResponseRepo, services::OrderResponseRepo,
};
use ::position::{
  entities::Position, interfaces::IPositionRepo, services::PositionRepo,
};
use ::rpc::{bot_mode::BotMode, exchanges::Exchanges};
use ::subscribe::nats::Client as Nats;

use crate::traits::Executor as ExecutorTrait;

use super::interfaces::{
  ICancelOrderRequestMaker, INewOrderRequestMaker, IOrderClient,
  IReverseOrderRequestMaker,
};
use super::services::{
  CancelOrderRequestMaker, NewOrderRequestMaker, OrderClient,
  ReverseOrderRequestMaker,
};

pub struct Executor {
  keychain: Arc<dyn IKeyChain + Send + Sync>,
  new_order_request_maker: Arc<dyn INewOrderRequestMaker + Send + Sync>,
  cancel_request_maker: Arc<dyn ICancelOrderRequestMaker + Send + Sync>,
  reverse_request_maker: Arc<dyn IReverseOrderRequestMaker + Send + Sync>,
  position_repo: Arc<dyn IPositionRepo + Send + Sync>,
  order_resp_repo: Arc<dyn IOrderResponseRepo + Send + Sync>,
  cli: Arc<dyn IOrderClient + Send + Sync>,
  broker: Nats,
}

impl Executor {
  pub async fn new(broker: &Nats, db: Database) -> ExecutionResult<Self> {
    let keychain = KeyChain::new(broker, db.clone()).await?;
    let new_order_request_maker = NewOrderRequestMaker::new();
    let cancel_request_maker = CancelOrderRequestMaker::new();
    let reverse_request_maker = ReverseOrderRequestMaker::new();
    let position_repo = PositionRepo::new(db.clone()).await;
    let order_resp_repo = OrderResponseRepo::new(db.clone()).await;
    let signer = Arc::new(APIKeySigner::new());
    let cli = OrderClient::new(signer.clone(), signer.clone())?;

    let me = Self {
      keychain: Arc::new(keychain),
      new_order_request_maker: Arc::new(new_order_request_maker),
      cancel_request_maker: Arc::new(cancel_request_maker),
      reverse_request_maker: Arc::new(reverse_request_maker),
      position_repo: Arc::new(position_repo),
      order_resp_repo: Arc::new(order_resp_repo),
      broker: broker.clone(),
      cli: Arc::new(cli),
    };
    return Ok(me);
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &self,
  ) -> ExecutionResult<BoxStream<'_, ExecutionResult<BookTicker>>> {
    let stream = try_stream! {
      let observer = TradeSubscriber::new(
      &self.broker,
    )
    .await?;
    let mut sub = observer.subscribe().await?;
      while let Some(book_ticker) = sub.next().await {
        yield book_ticker;
      }
    };
    return Ok(Box::pin(stream));
  }
  async fn create_order(
    &self,
    bot_id: ObjectId,
    api_key_id: ObjectId,
    symbol: String,
    price: Option<Float>,
    budget: Float,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<ObjectId> {
    let position_group = Position::new(bot_id, BotMode::Live, &symbol);
    self.position_repo.save(&[&position_group]).await?;
    let api_key =
      Arc::new(self.keychain.get(Exchanges::Binance, api_key_id).await?);
    let req =
      self
        .new_order_request_maker
        .build(symbol, budget, price, order_option);
    let resp_defers: Vec<BoxFuture<ExecutionResult<()>>> = req
      .into_iter()
      .map(|req| {
        let req = Arc::new(req);
        return self.cli.new_order(api_key.clone(), req);
      })
      .map(|fut| {
        let repo = self.order_resp_repo.clone();
        let gid = position_group.entry_gid.clone();
        return fut
          .and_then(move |mut payload| async move {
            payload.gid = Some(gid);
            repo.save(&[&payload]).await?;
            return Ok(());
          })
          .boxed();
      })
      .collect();
    let _ = try_join_all(resp_defers).await?;
    return Ok(position_group.entry_gid);
  }

  async fn remove_order(
    &self,
    api_key_id: ObjectId,
    gid: ObjectId,
  ) -> ExecutionResult<ExecutionSummary> {
    let api_key =
      Arc::new(self.keychain.get(Exchanges::Binance, api_key_id).await?);
    let position = self.position_repo.get(&gid).await?;
    let mut order_resp_cur = self
      .order_resp_repo
      .find_by_entry_position(&position)
      .await?;
    let mut order_cancel_vec = vec![];
    let mut position_reverse_vec = vec![];
    while let Some(pos_result) = order_resp_cur.next().await {
      let pos = match pos_result {
        Ok(pos) => pos,
        Err(e) => {
          error!(
            error = as_error!(e),
            gid = gid.to_hex(),
            exchange = as_serde!(Exchanges::Binance);
            "Error occured on retriving the order response from DB. Skipping..."
          );
          continue;
        }
      };
      let pos = Arc::new(pos);
      // Cancel Order
      let maker = self.cancel_request_maker.clone();
      let pos = pos.clone();
      let cancel_order = Arc::new(maker.build(pos.as_ref())?);
      let order_resp_repo = self.order_resp_repo.clone();
      order_cancel_vec.push(
        self
          .cli
          .cancel_order(api_key.clone(), cancel_order)
          .and_then(|mut order_resp| async move {
            order_resp.gid = Some(position.exit_gid.clone());
            let _ = order_resp_repo.save(&[&order_resp]).await?;
            return Ok(());
          })
          .boxed(),
      );
      if pos.fills.is_some() {
        // Sell the position
        let req = Arc::new(self.reverse_request_maker.build(pos.as_ref())?);
        let pos: Order = pos.as_ref().into();
        let pos_pur_price: OrderInner = pos.clone().sum();
        let order_resp_repo = self.order_resp_repo.clone();
        position_reverse_vec.push(
          self
            .cli
            .new_order(api_key.clone(), req)
            .and_then(|mut resp| async move {
              resp.gid = Some(position.exit_gid.clone());
              let _ = order_resp_repo.save(&[&resp]).await?;
              let resp: Order = (&resp).into();
              let rev_pos_price: OrderInner = resp.sum();
              return Ok((pos_pur_price, rev_pos_price));
            })
            .boxed(),
        );
      };
    }
    let (cancel_resp, reverse_resp) = join(
      try_join_all(order_cancel_vec),
      try_join_all(position_reverse_vec),
    )
    .await;
    let (_, reverse_resp) = (cancel_resp?, reverse_resp?);
    let mut pur_order = OrderInner::default();
    let mut sell_order = OrderInner::default();
    for (pur, sell) in reverse_resp.iter() {
      pur_order += pur;
      sell_order += sell;
    }
    return Ok(ExecutionSummary::calculate_profit(&sell_order, &pur_order));
  }
}
