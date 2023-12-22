use ::futures::stream::BoxStream;
use ::rpc::symbol_info::SymbolInfo;

pub type ListSymbolStream = BoxStream<'static, SymbolInfo>;
