use ::nats::asynk::Connection as NatsCon;

pub struct Executor {
  con: NatsCon,
}
