#!/usr/bin/env node

const { JsOrderBook, hello } = require('./index.js');

console.log('=== Basic Node.js Test ===');
console.log('Hello function:', hello());

const book = new JsOrderBook('TEST');
console.log('Created book for symbol:', book.symbol());
console.log('Best bid:', book.bestBid());
console.log('Best ask:', book.bestAsk());

console.log('Adding limit order...');
const orderId = book.addLimitOrder(100.0, 10.0, 'buy', 'gtc');
console.log('Order ID:', orderId);
console.log('Best bid after order:', book.bestBid());

console.log('=== Test Complete ===');