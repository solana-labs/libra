// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
#![allow(unused_variables)]
use futures::Future;


pub trait MempoolClientTrait : Clone + Send + Sync {

    fn add_transaction_with_validation(&self, req: &super::mempool::AddTransactionWithValidationRequest) -> ::grpcio::Result<super::mempool::AddTransactionWithValidationResponse> {
        unimplemented!();
    }
    fn add_transaction_with_validation_async(&self, req: &super::mempool::AddTransactionWithValidationRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::AddTransactionWithValidationResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }

    fn get_block(&self, req: &super::mempool::GetBlockRequest) -> ::grpcio::Result<super::mempool::GetBlockResponse> {
        unimplemented!();
    }
    fn get_block_async(&self, req: &super::mempool::GetBlockRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::GetBlockResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }

    fn commit_transactions(&self, req: &super::mempool::CommitTransactionsRequest) -> ::grpcio::Result<super::mempool::CommitTransactionsResponse> {
        unimplemented!();
    }
    fn commit_transactions_async(&self, req: &super::mempool::CommitTransactionsRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::CommitTransactionsResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }

    fn health_check(&self, req: &super::mempool::HealthCheckRequest) -> ::grpcio::Result<super::mempool::HealthCheckResponse> {
        unimplemented!();
    }
    fn health_check_async(&self, req: &super::mempool::HealthCheckRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::HealthCheckResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }
}

impl MempoolClientTrait for super::mempool_grpc::MempoolClient {

    fn add_transaction_with_validation(&self, req: &super::mempool::AddTransactionWithValidationRequest) -> ::grpcio::Result<super::mempool::AddTransactionWithValidationResponse> {
        self.add_transaction_with_validation(req)
    }
    fn add_transaction_with_validation_async(&self, req: &super::mempool::AddTransactionWithValidationRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::AddTransactionWithValidationResponse, Error=::grpcio::Error> + Send>> {
        match self.add_transaction_with_validation_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }

    fn get_block(&self, req: &super::mempool::GetBlockRequest) -> ::grpcio::Result<super::mempool::GetBlockResponse> {
        self.get_block(req)
    }
    fn get_block_async(&self, req: &super::mempool::GetBlockRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::GetBlockResponse, Error=::grpcio::Error> + Send>> {
        match self.get_block_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }

    fn commit_transactions(&self, req: &super::mempool::CommitTransactionsRequest) -> ::grpcio::Result<super::mempool::CommitTransactionsResponse> {
        self.commit_transactions(req)
    }
    fn commit_transactions_async(&self, req: &super::mempool::CommitTransactionsRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::CommitTransactionsResponse, Error=::grpcio::Error> + Send>> {
        match self.commit_transactions_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }

    fn health_check(&self, req: &super::mempool::HealthCheckRequest) -> ::grpcio::Result<super::mempool::HealthCheckResponse> {
        self.health_check(req)
    }
    fn health_check_async(&self, req: &super::mempool::HealthCheckRequest) -> ::grpcio::Result<Box<Future<Item=super::mempool::HealthCheckResponse, Error=::grpcio::Error> + Send>> {
        match self.health_check_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }
}
