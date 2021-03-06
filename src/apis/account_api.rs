/* 
 * Aeternity Epoch
 *
 * This is the [Aeternity](https://www.aeternity.com/) Epoch API.
 *
 * OpenAPI spec version: 0.25.0
 * Contact: apiteam@aeternity.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct AccountApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AccountApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AccountApiClient<C> {
        AccountApiClient {
            configuration: configuration,
        }
    }
}

pub trait AccountApi {
    fn get_account_by_pubkey(&self, pubkey: &str) -> Box<Future<Item = ::models::Account, Error = Error>>;
    fn get_pending_account_transactions_by_pubkey(&self, pubkey: &str) -> Box<Future<Item = ::models::GenericTxs, Error = Error>>;
}


impl<C: hyper::client::Connect>AccountApi for AccountApiClient<C> {
    fn get_account_by_pubkey(&self, pubkey: &str) -> Box<Future<Item = ::models::Account, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/accounts/{pubkey}", configuration.base_path, pubkey=pubkey);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::Account, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_pending_account_transactions_by_pubkey(&self, pubkey: &str) -> Box<Future<Item = ::models::GenericTxs, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/accounts/{pubkey}/transactions/pending", configuration.base_path, pubkey=pubkey);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::GenericTxs, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
