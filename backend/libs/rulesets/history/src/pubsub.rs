use ::subscribe::pubsub;

use super::entities::KlineFetchStatus;

pubsub!(pub, FetchStatusPubSub, KlineFetchStatus, "kline.progress");
