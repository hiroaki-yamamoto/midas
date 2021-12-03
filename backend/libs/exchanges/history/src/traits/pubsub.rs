use ::subscribe::pubsub;

use crate::entities::KlineFetchStatus;

pubsub!(pub, FetchStatusPubSub, KlineFetchStatus, "kline.progress");
