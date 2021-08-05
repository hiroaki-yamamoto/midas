/**
 * The rate of reinvestment. The bot will add the profit multiplied this rate
 * to the trading amount. i.e. (ra) = (profit) x (rr), where ra = re-invest amount,
 * and rr = re-investment rate.
 * If you don't want re-invest, set this value to 0.
 */
const reInvestRate = 1.0;

/**
 * Entry watch point to get interested in put position.
 * After the result of this function get true, the bot starts watching the market.
 *
 * Note that this function doesn't place the position actually. After this function
 * returns true, and the entryPoint gets true, the position will be actually placed.
 * @param {PriceVolume} current - Current Price
 * @returns {boolean} - Return true when the market has reached watchpoint, and
 *  return false otherwise.
 */
function entryWatchPoint(current: PriceVolume): boolean {
  // Write entry watchpoint condition here.
  return false;
}

/**
 * Entry point to place the order. If this function returns true, the position
 * will be placed.
 *
 * Note that this function is invoked AFTER entryWatchPoint.
 * @param {PriceVolume} current - Current Price
 * @param {PriceVolume} watch - The price when the bot started watching
 * @returns {boolean} - Return true when the market has reached entrypoint, and
 *  return false otherwise.
 */
function entryPoint(current: PriceVolume, watch: PriceVolume): boolean {
  // Write entryoiint condition here.
  return false;
}

/**
 * Entry watch point to get interested in pull position.
 * After the result of this function get true, the bot starts watching the market
 * to exit the position.
 *
 * Note that this function doesn't exit the position actually. After this function
 * returns true, and the exitPoint gets true, the position will be actually exited.
 * @param {PriceVolume} current - Current Price
 * @returns {boolean} - Return true when the market has reached watchpoint, and
 *  return false otherwise.
 */
function exitWatchPoint(current: PriceVolume): boolean {
  // Write extit watch point here.
  return false;
}

/**
 * Exit point to place the order. If this function returns true, the position
 * will be exited.
 *
 * Note that this function is invoked AFTER exitWatchPoint.
 * @param {PriceVolume} current - Current Price
 * @param {PriceVolume} watch - The price when the bot started watching
 * @returns {boolean} - Return true when the market has reached exit point, and
 *  return false otherwise
 */
function exitPoint(current: PriceVolume, watch: PriceVolume): boolean {
  // Write exit point here.
  return false;
}
