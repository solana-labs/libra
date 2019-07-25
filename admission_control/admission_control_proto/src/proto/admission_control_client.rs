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


pub trait AdmissionControlClientTrait : Clone + Send + Sync {

    fn submit_transaction(&self, req: &super::admission_control::SubmitTransactionRequest) -> ::grpcio::Result<super::admission_control::SubmitTransactionResponse> {
        unimplemented!();
    }
    fn submit_transaction_async(&self, req: &super::admission_control::SubmitTransactionRequest) -> ::grpcio::Result<Box<Future<Item=super::admission_control::SubmitTransactionResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }

    fn update_to_latest_ledger(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<super::get_with_proof::UpdateToLatestLedgerResponse> {
        unimplemented!();
    }
    fn update_to_latest_ledger_async(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<Box<Future<Item=super::get_with_proof::UpdateToLatestLedgerResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }
}

impl AdmissionControlClientTrait for super::admission_control_grpc::AdmissionControlClient {

    fn submit_transaction(&self, req: &super::admission_control::SubmitTransactionRequest) -> ::grpcio::Result<super::admission_control::SubmitTransactionResponse> {
        self.submit_transaction(req)
    }
    fn submit_transaction_async(&self, req: &super::admission_control::SubmitTransactionRequest) -> ::grpcio::Result<Box<Future<Item=super::admission_control::SubmitTransactionResponse, Error=::grpcio::Error> + Send>> {
        match self.submit_transaction_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }

    fn update_to_latest_ledger(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<super::get_with_proof::UpdateToLatestLedgerResponse> {
        self.update_to_latest_ledger(req)
    }
    fn update_to_latest_ledger_async(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<Box<Future<Item=super::get_with_proof::UpdateToLatestLedgerResponse, Error=::grpcio::Error> + Send>> {
        match self.update_to_latest_ledger_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }
}
