use ::futures::stream::BoxStream;
use ::rpc::symbols::SymbolInfo;

pub type ListSymbolStream = BoxStream<'static, SymbolInfo>;
