/**
 * @fileoverview gRPC-Web generated client stub for services
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.services = require('./bot_manager_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.services.BotManagerClient =
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
proto.services.BotManagerPromiseClient =
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
 *   !proto.services.BotInfoListRequest,
 *   !proto.services.BotInfoList>}
 */
const methodDescriptor_BotManager_ListBotInfo = new grpc.web.MethodDescriptor(
  '/services.BotManager/ListBotInfo',
  grpc.web.MethodType.UNARY,
  proto.services.BotInfoListRequest,
  proto.services.BotInfoList,
  /**
   * @param {!proto.services.BotInfoListRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.services.BotInfoList.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.services.BotInfoListRequest,
 *   !proto.services.BotInfoList>}
 */
const methodInfo_BotManager_ListBotInfo = new grpc.web.AbstractClientBase.MethodInfo(
  proto.services.BotInfoList,
  /**
   * @param {!proto.services.BotInfoListRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.services.BotInfoList.deserializeBinary
);


/**
 * @param {!proto.services.BotInfoListRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.services.BotInfoList)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.services.BotInfoList>|undefined}
 *     The XHR Node Readable Stream
 */
proto.services.BotManagerClient.prototype.listBotInfo =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/services.BotManager/ListBotInfo',
      request,
      metadata || {},
      methodDescriptor_BotManager_ListBotInfo,
      callback);
};


/**
 * @param {!proto.services.BotInfoListRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.services.BotInfoList>}
 *     Promise that resolves to the response
 */
proto.services.BotManagerPromiseClient.prototype.listBotInfo =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/services.BotManager/ListBotInfo',
      request,
      metadata || {},
      methodDescriptor_BotManager_ListBotInfo);
};


module.exports = proto.services;

