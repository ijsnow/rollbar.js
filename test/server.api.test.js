'use strict';

var assert = require('assert');
var vows = require('vows');
var sinon = require('sinon');

process.env.NODE_ENV = process.env.NODE_ENV || 'test-node-env';

var API = require('../src/api');

var rust = require('../api/index.node');

function TestTransportGenerator() {
  var TestTransport = function(callbackError, callbackResponse) {
    this.postArgs = [];
    this.callbackError = callbackError;
    this.callbackResponse = callbackResponse;
  };

  TestTransport.prototype.post = function() {
    var args = arguments;
    this.postArgs.push(args);
    var callback = args[args.length-1];
    if (typeof callback === 'function') {
      callback(this.callbackError, this.callbackResponse);
    }
  };

  return TestTransport;
}

vows.describe('api')
  .addBatch({
      'with native url parser': {
        topic: function() {
          var transport = new (TestTransportGenerator())();
          var accessToken = 'abc123';
          var endpoint = 'http://woo.foo.com/api/42';
          var api = new API({ accessToken, endpoint }, transport, {parse: rust.parseUrl});

          return api;
        },
        'parsed correct info': function(api) {
          assert.equal(api.transportOptions.hostname, 'woo.foo.com');
          assert.equal(api.transportOptions.path, '/api/42');
          assert.equal(api.transportOptions.protocol, 'http:');
          assert.equal(api.transportOptions.port, '443');
        }
    },
  }).export(module, {error: false});
