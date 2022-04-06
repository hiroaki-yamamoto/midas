use ::warp::filters::BoxedFilter;
use ::warp::Filter;
use ::warp::Reply;

pub fn probe() -> BoxedFilter<(impl Reply,)> {
  ::warp::get()
    .and(::warp::path("probe"))
    .and(::warp::path("live"))
    .map(|| ::warp::reply())
    .boxed()
}
