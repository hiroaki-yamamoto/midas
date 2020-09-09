/**
 * @fileoverview gRPC-Web generated client stub for historical
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var entities_pb = require('./entities_pb.js')

var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js')
const proto = {};
proto.historical = require('./historical_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.historical.HistChartClient =
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
proto.historical.HistChartPromiseClient =
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
 *   !proto.historical.HistChartFetchReq,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_HistChart_sync = new grpc.web.MethodDescriptor(
  '/historical.HistChart/sync',
  grpc.web.MethodType.UNARY,
  proto.historical.HistChartFetchReq,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.historical.HistChartFetchReq} request
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
 *   !proto.historical.HistChartFetchReq,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_HistChart_sync = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.historical.HistChartFetchReq} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.historical.HistChartFetchReq} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.historical.HistChartClient.prototype.sync =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/historical.HistChart/sync',
      request,
      metadata || {},
      methodDescriptor_HistChart_sync,
      callback);
};


/**
 * @param {!proto.historical.HistChartFetchReq} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.historical.HistChartPromiseClient.prototype.sync =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/historical.HistChart/sync',
      request,
      metadata || {},
      methodDescriptor_HistChart_sync);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.google.protobuf.Empty,
 *   !proto.historical.HistChartProg>}
 */
const methodDescriptor_HistChart_subscribe = new grpc.web.MethodDescriptor(
  '/historical.HistChart/subscribe',
  grpc.web.MethodType.SERVER_STREAMING,
  google_protobuf_empty_pb.Empty,
  proto.historical.HistChartProg,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.historical.HistChartProg.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.google.protobuf.Empty,
 *   !proto.historical.HistChartProg>}
 */
const methodInfo_HistChart_subscribe = new grpc.web.AbstractClientBase.MethodInfo(
  proto.historical.HistChartProg,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.historical.HistChartProg.deserializeBinary
);


/**
 * @param {!proto.google.protobuf.Empty} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.historical.HistChartProg>}
 *     The XHR Node Readable Stream
 */
proto.historical.HistChartClient.prototype.subscribe =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/historical.HistChart/subscribe',
      request,
      metadata || {},
      methodDescriptor_HistChart_subscribe);
};


/**
 * @param {!proto.google.protobuf.Empty} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.historical.HistChartProg>}
 *     The XHR Node Readable Stream
 */
proto.historical.HistChartPromiseClient.prototype.subscribe =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/historical.HistChart/subscribe',
      request,
      metadata || {},
      methodDescriptor_HistChart_subscribe);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.historical.StopRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_HistChart_stop = new grpc.web.MethodDescriptor(
  '/historical.HistChart/stop',
  grpc.web.MethodType.UNARY,
  proto.historical.StopRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.historical.StopRequest} request
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
 *   !proto.historical.StopRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_HistChart_stop = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.historical.StopRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.historical.StopRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.historical.HistChartClient.prototype.stop =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/historical.HistChart/stop',
      request,
      metadata || {},
      methodDescriptor_HistChart_stop,
      callback);
};


/**
 * @param {!proto.historical.StopRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.historical.HistChartPromiseClient.prototype.stop =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/historical.HistChart/stop',
      request,
      metadata || {},
      methodDescriptor_HistChart_stop);
};


module.exports = proto.historical;

