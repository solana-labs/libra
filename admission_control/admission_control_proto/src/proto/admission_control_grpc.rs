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

const METHOD_ADMISSION_CONTROL_SUBMIT_TRANSACTION: ::grpcio::Method<super::admission_control::SubmitTransactionRequest, super::admission_control::SubmitTransactionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/admission_control.AdmissionControl/SubmitTransaction",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ADMISSION_CONTROL_UPDATE_TO_LATEST_LEDGER: ::grpcio::Method<super::get_with_proof::UpdateToLatestLedgerRequest, super::get_with_proof::UpdateToLatestLedgerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/admission_control.AdmissionControl/UpdateToLatestLedger",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AdmissionControlClient {
    client: ::grpcio::Client,
}

impl AdmissionControlClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AdmissionControlClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn submit_transaction_opt(&self, req: &super::admission_control::SubmitTransactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::admission_control::SubmitTransactionResponse> {
        self.client.unary_call(&METHOD_ADMISSION_CONTROL_SUBMIT_TRANSACTION, req, opt)
    }

    pub fn submit_transaction(&self, req: &super::admission_control::SubmitTransactionRequest) -> ::grpcio::Result<super::admission_control::SubmitTransactionResponse> {
        self.submit_transaction_opt(req, ::grpcio::CallOption::default())
    }

    pub fn submit_transaction_async_opt(&self, req: &super::admission_control::SubmitTransactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::admission_control::SubmitTransactionResponse>> {
        self.client.unary_call_async(&METHOD_ADMISSION_CONTROL_SUBMIT_TRANSACTION, req, opt)
    }

    pub fn submit_transaction_async(&self, req: &super::admission_control::SubmitTransactionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::admission_control::SubmitTransactionResponse>> {
        self.submit_transaction_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_to_latest_ledger_opt(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::get_with_proof::UpdateToLatestLedgerResponse> {
        self.client.unary_call(&METHOD_ADMISSION_CONTROL_UPDATE_TO_LATEST_LEDGER, req, opt)
    }

    pub fn update_to_latest_ledger(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<super::get_with_proof::UpdateToLatestLedgerResponse> {
        self.update_to_latest_ledger_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_to_latest_ledger_async_opt(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::get_with_proof::UpdateToLatestLedgerResponse>> {
        self.client.unary_call_async(&METHOD_ADMISSION_CONTROL_UPDATE_TO_LATEST_LEDGER, req, opt)
    }

    pub fn update_to_latest_ledger_async(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::get_with_proof::UpdateToLatestLedgerResponse>> {
        self.update_to_latest_ledger_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AdmissionControl {
    fn submit_transaction(&mut self, ctx: ::grpcio::RpcContext, req: super::admission_control::SubmitTransactionRequest, sink: ::grpcio::UnarySink<super::admission_control::SubmitTransactionResponse>);
    fn update_to_latest_ledger(&mut self, ctx: ::grpcio::RpcContext, req: super::get_with_proof::UpdateToLatestLedgerRequest, sink: ::grpcio::UnarySink<super::get_with_proof::UpdateToLatestLedgerResponse>);
}

pub fn create_admission_control<S: AdmissionControl + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ADMISSION_CONTROL_SUBMIT_TRANSACTION, move |ctx, req, resp| {
        instance.submit_transaction(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ADMISSION_CONTROL_UPDATE_TO_LATEST_LEDGER, move |ctx, req, resp| {
        instance.update_to_latest_ledger(ctx, req, resp)
    });
    builder.build()
}
