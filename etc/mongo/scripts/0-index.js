const con = new Mongo();
const db = con.getDB("midas");

const binance_hist_col = db.binance.history;
const binance_symb_col = db.binance.symbol;

const binance_hist_indices = binance_hist_col.getIndexes();
const binance_symb_indices = binance_symb_col.getIndexes();

['open_time', 'close_time', 'symbol'].forEach((name) => {
  const target = binance_hist_indices.filter((index) => {
    return index.name === name;
  });
  if (target.length < 1) {
    let obj = {};
    obj[name] = 1;
    binance_hist_col.createIndex(obj, {name});
  }
});

['symbol'].forEach((name) => {
  const target = binance_symb_indices.filter((index) => {
    return index.name === name;
  });
  if (target.length < 1) {
    let obj = {};
    obj[name] = 1;
    binance_symb_col.createIndex(obj, { name });
  }
});
