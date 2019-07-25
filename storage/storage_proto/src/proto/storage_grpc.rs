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

const METHOD_STORAGE_SAVE_TRANSACTIONS: ::grpcio::Method<super::storage::SaveTransactionsRequest, super::storage::SaveTransactionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/storage.Storage/SaveTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STORAGE_UPDATE_TO_LATEST_LEDGER: ::grpcio::Method<super::get_with_proof::UpdateToLatestLedgerRequest, super::get_with_proof::UpdateToLatestLedgerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/storage.Storage/UpdateToLatestLedger",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STORAGE_GET_TRANSACTIONS: ::grpcio::Method<super::storage::GetTransactionsRequest, super::storage::GetTransactionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/storage.Storage/GetTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STORAGE_GET_ACCOUNT_STATE_WITH_PROOF_BY_STATE_ROOT: ::grpcio::Method<super::storage::GetAccountStateWithProofByStateRootRequest, super::storage::GetAccountStateWithProofByStateRootResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/storage.Storage/GetAccountStateWithProofByStateRoot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STORAGE_GET_EXECUTOR_STARTUP_INFO: ::grpcio::Method<super::storage::GetExecutorStartupInfoRequest, super::storage::GetExecutorStartupInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/storage.Storage/GetExecutorStartupInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StorageClient {
    client: ::grpcio::Client,
}

impl StorageClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StorageClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn save_transactions_opt(&self, req: &super::storage::SaveTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::storage::SaveTransactionsResponse> {
        self.client.unary_call(&METHOD_STORAGE_SAVE_TRANSACTIONS, req, opt)
    }

    pub fn save_transactions(&self, req: &super::storage::SaveTransactionsRequest) -> ::grpcio::Result<super::storage::SaveTransactionsResponse> {
        self.save_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn save_transactions_async_opt(&self, req: &super::storage::SaveTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::SaveTransactionsResponse>> {
        self.client.unary_call_async(&METHOD_STORAGE_SAVE_TRANSACTIONS, req, opt)
    }

    pub fn save_transactions_async(&self, req: &super::storage::SaveTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::SaveTransactionsResponse>> {
        self.save_transactions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_to_latest_ledger_opt(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::get_with_proof::UpdateToLatestLedgerResponse> {
        self.client.unary_call(&METHOD_STORAGE_UPDATE_TO_LATEST_LEDGER, req, opt)
    }

    pub fn update_to_latest_ledger(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<super::get_with_proof::UpdateToLatestLedgerResponse> {
        self.update_to_latest_ledger_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_to_latest_ledger_async_opt(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::get_with_proof::UpdateToLatestLedgerResponse>> {
        self.client.unary_call_async(&METHOD_STORAGE_UPDATE_TO_LATEST_LEDGER, req, opt)
    }

    pub fn update_to_latest_ledger_async(&self, req: &super::get_with_proof::UpdateToLatestLedgerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::get_with_proof::UpdateToLatestLedgerResponse>> {
        self.update_to_latest_ledger_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_transactions_opt(&self, req: &super::storage::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::storage::GetTransactionsResponse> {
        self.client.unary_call(&METHOD_STORAGE_GET_TRANSACTIONS, req, opt)
    }

    pub fn get_transactions(&self, req: &super::storage::GetTransactionsRequest) -> ::grpcio::Result<super::storage::GetTransactionsResponse> {
        self.get_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_transactions_async_opt(&self, req: &super::storage::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::GetTransactionsResponse>> {
        self.client.unary_call_async(&METHOD_STORAGE_GET_TRANSACTIONS, req, opt)
    }

    pub fn get_transactions_async(&self, req: &super::storage::GetTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::GetTransactionsResponse>> {
        self.get_transactions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_account_state_with_proof_by_state_root_opt(&self, req: &super::storage::GetAccountStateWithProofByStateRootRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::storage::GetAccountStateWithProofByStateRootResponse> {
        self.client.unary_call(&METHOD_STORAGE_GET_ACCOUNT_STATE_WITH_PROOF_BY_STATE_ROOT, req, opt)
    }

    pub fn get_account_state_with_proof_by_state_root(&self, req: &super::storage::GetAccountStateWithProofByStateRootRequest) -> ::grpcio::Result<super::storage::GetAccountStateWithProofByStateRootResponse> {
        self.get_account_state_with_proof_by_state_root_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_account_state_with_proof_by_state_root_async_opt(&self, req: &super::storage::GetAccountStateWithProofByStateRootRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::GetAccountStateWithProofByStateRootResponse>> {
        self.client.unary_call_async(&METHOD_STORAGE_GET_ACCOUNT_STATE_WITH_PROOF_BY_STATE_ROOT, req, opt)
    }

    pub fn get_account_state_with_proof_by_state_root_async(&self, req: &super::storage::GetAccountStateWithProofByStateRootRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::GetAccountStateWithProofByStateRootResponse>> {
        self.get_account_state_with_proof_by_state_root_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_executor_startup_info_opt(&self, req: &super::storage::GetExecutorStartupInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::storage::GetExecutorStartupInfoResponse> {
        self.client.unary_call(&METHOD_STORAGE_GET_EXECUTOR_STARTUP_INFO, req, opt)
    }

    pub fn get_executor_startup_info(&self, req: &super::storage::GetExecutorStartupInfoRequest) -> ::grpcio::Result<super::storage::GetExecutorStartupInfoResponse> {
        self.get_executor_startup_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_executor_startup_info_async_opt(&self, req: &super::storage::GetExecutorStartupInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::GetExecutorStartupInfoResponse>> {
        self.client.unary_call_async(&METHOD_STORAGE_GET_EXECUTOR_STARTUP_INFO, req, opt)
    }

    pub fn get_executor_startup_info_async(&self, req: &super::storage::GetExecutorStartupInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::storage::GetExecutorStartupInfoResponse>> {
        self.get_executor_startup_info_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Storage {
    fn save_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::storage::SaveTransactionsRequest, sink: ::grpcio::UnarySink<super::storage::SaveTransactionsResponse>);
    fn update_to_latest_ledger(&mut self, ctx: ::grpcio::RpcContext, req: super::get_with_proof::UpdateToLatestLedgerRequest, sink: ::grpcio::UnarySink<super::get_with_proof::UpdateToLatestLedgerResponse>);
    fn get_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::storage::GetTransactionsRequest, sink: ::grpcio::UnarySink<super::storage::GetTransactionsResponse>);
    fn get_account_state_with_proof_by_state_root(&mut self, ctx: ::grpcio::RpcContext, req: super::storage::GetAccountStateWithProofByStateRootRequest, sink: ::grpcio::UnarySink<super::storage::GetAccountStateWithProofByStateRootResponse>);
    fn get_executor_startup_info(&mut self, ctx: ::grpcio::RpcContext, req: super::storage::GetExecutorStartupInfoRequest, sink: ::grpcio::UnarySink<super::storage::GetExecutorStartupInfoResponse>);
}

pub fn create_storage<S: Storage + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STORAGE_SAVE_TRANSACTIONS, move |ctx, req, resp| {
        instance.save_transactions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STORAGE_UPDATE_TO_LATEST_LEDGER, move |ctx, req, resp| {
        instance.update_to_latest_ledger(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STORAGE_GET_TRANSACTIONS, move |ctx, req, resp| {
        instance.get_transactions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STORAGE_GET_ACCOUNT_STATE_WITH_PROOF_BY_STATE_ROOT, move |ctx, req, resp| {
        instance.get_account_state_with_proof_by_state_root(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STORAGE_GET_EXECUTOR_STARTUP_INFO, move |ctx, req, resp| {
        instance.get_executor_startup_info(ctx, req, resp)
    });
    builder.build()
}
