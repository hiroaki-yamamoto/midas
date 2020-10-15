/**
 * @fileoverview gRPC-Web generated client stub for symbol
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js')

var entities_pb = require('./entities_pb.js')
const proto = {};
proto.symbol = require('./symbol_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.symbol.SymbolClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.symbol.SymbolPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.symbol.RefreshRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_Symbol_refresh = new grpc.web.MethodDescriptor(
  '/symbol.Symbol/refresh',
  grpc.web.MethodType.UNARY,
  proto.symbol.RefreshRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.symbol.RefreshRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.symbol.RefreshRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_Symbol_refresh = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.symbol.RefreshRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.symbol.RefreshRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.symbol.SymbolClient.prototype.refresh =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/symbol.Symbol/refresh',
      request,
      metadata || {},
      methodDescriptor_Symbol_refresh,
      callback);
};


/**
 * @param {!proto.symbol.RefreshRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.symbol.SymbolPromiseClient.prototype.refresh =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/symbol.Symbol/refresh',
      request,
      metadata || {},
      methodDescriptor_Symbol_refresh);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.symbol.QueryRequest,
 *   !proto.symbol.QueryResponse>}
 */
const methodDescriptor_Symbol_query = new grpc.web.MethodDescriptor(
  '/symbol.Symbol/query',
  grpc.web.MethodType.UNARY,
  proto.symbol.QueryRequest,
  proto.symbol.QueryResponse,
  /**
   * @param {!proto.symbol.QueryRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.symbol.QueryResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.symbol.QueryRequest,
 *   !proto.symbol.QueryResponse>}
 */
const methodInfo_Symbol_query = new grpc.web.AbstractClientBase.MethodInfo(
  proto.symbol.QueryResponse,
  /**
   * @param {!proto.symbol.QueryRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.symbol.QueryResponse.deserializeBinary
);


/**
 * @param {!proto.symbol.QueryRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.symbol.QueryResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.symbol.QueryResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.symbol.SymbolClient.prototype.query =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/symbol.Symbol/query',
      request,
      metadata || {},
      methodDescriptor_Symbol_query,
      callback);
};


/**
 * @param {!proto.symbol.QueryRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.symbol.QueryResponse>}
 *     Promise that resolves to the response
 */
proto.symbol.SymbolPromiseClient.prototype.query =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/symbol.Symbol/query',
      request,
      metadata || {},
      methodDescriptor_Symbol_query);
};


module.exports = proto.symbol;

