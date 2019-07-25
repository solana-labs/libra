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

const METHOD_MEMPOOL_ADD_TRANSACTION_WITH_VALIDATION: ::grpcio::Method<super::mempool::AddTransactionWithValidationRequest, super::mempool::AddTransactionWithValidationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mempool.Mempool/AddTransactionWithValidation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MEMPOOL_GET_BLOCK: ::grpcio::Method<super::mempool::GetBlockRequest, super::mempool::GetBlockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mempool.Mempool/GetBlock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MEMPOOL_COMMIT_TRANSACTIONS: ::grpcio::Method<super::mempool::CommitTransactionsRequest, super::mempool::CommitTransactionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mempool.Mempool/CommitTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MEMPOOL_HEALTH_CHECK: ::grpcio::Method<super::mempool::HealthCheckRequest, super::mempool::HealthCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mempool.Mempool/HealthCheck",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MempoolClient {
    client: ::grpcio::Client,
}

impl MempoolClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MempoolClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn add_transaction_with_validation_opt(&self, req: &super::mempool::AddTransactionWithValidationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mempool::AddTransactionWithValidationResponse> {
        self.client.unary_call(&METHOD_MEMPOOL_ADD_TRANSACTION_WITH_VALIDATION, req, opt)
    }

    pub fn add_transaction_with_validation(&self, req: &super::mempool::AddTransactionWithValidationRequest) -> ::grpcio::Result<super::mempool::AddTransactionWithValidationResponse> {
        self.add_transaction_with_validation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_transaction_with_validation_async_opt(&self, req: &super::mempool::AddTransactionWithValidationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::AddTransactionWithValidationResponse>> {
        self.client.unary_call_async(&METHOD_MEMPOOL_ADD_TRANSACTION_WITH_VALIDATION, req, opt)
    }

    pub fn add_transaction_with_validation_async(&self, req: &super::mempool::AddTransactionWithValidationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::AddTransactionWithValidationResponse>> {
        self.add_transaction_with_validation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_block_opt(&self, req: &super::mempool::GetBlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mempool::GetBlockResponse> {
        self.client.unary_call(&METHOD_MEMPOOL_GET_BLOCK, req, opt)
    }

    pub fn get_block(&self, req: &super::mempool::GetBlockRequest) -> ::grpcio::Result<super::mempool::GetBlockResponse> {
        self.get_block_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_block_async_opt(&self, req: &super::mempool::GetBlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::GetBlockResponse>> {
        self.client.unary_call_async(&METHOD_MEMPOOL_GET_BLOCK, req, opt)
    }

    pub fn get_block_async(&self, req: &super::mempool::GetBlockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::GetBlockResponse>> {
        self.get_block_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_transactions_opt(&self, req: &super::mempool::CommitTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mempool::CommitTransactionsResponse> {
        self.client.unary_call(&METHOD_MEMPOOL_COMMIT_TRANSACTIONS, req, opt)
    }

    pub fn commit_transactions(&self, req: &super::mempool::CommitTransactionsRequest) -> ::grpcio::Result<super::mempool::CommitTransactionsResponse> {
        self.commit_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_transactions_async_opt(&self, req: &super::mempool::CommitTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::CommitTransactionsResponse>> {
        self.client.unary_call_async(&METHOD_MEMPOOL_COMMIT_TRANSACTIONS, req, opt)
    }

    pub fn commit_transactions_async(&self, req: &super::mempool::CommitTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::CommitTransactionsResponse>> {
        self.commit_transactions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn health_check_opt(&self, req: &super::mempool::HealthCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mempool::HealthCheckResponse> {
        self.client.unary_call(&METHOD_MEMPOOL_HEALTH_CHECK, req, opt)
    }

    pub fn health_check(&self, req: &super::mempool::HealthCheckRequest) -> ::grpcio::Result<super::mempool::HealthCheckResponse> {
        self.health_check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn health_check_async_opt(&self, req: &super::mempool::HealthCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::HealthCheckResponse>> {
        self.client.unary_call_async(&METHOD_MEMPOOL_HEALTH_CHECK, req, opt)
    }

    pub fn health_check_async(&self, req: &super::mempool::HealthCheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mempool::HealthCheckResponse>> {
        self.health_check_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Mempool {
    fn add_transaction_with_validation(&mut self, ctx: ::grpcio::RpcContext, req: super::mempool::AddTransactionWithValidationRequest, sink: ::grpcio::UnarySink<super::mempool::AddTransactionWithValidationResponse>);
    fn get_block(&mut self, ctx: ::grpcio::RpcContext, req: super::mempool::GetBlockRequest, sink: ::grpcio::UnarySink<super::mempool::GetBlockResponse>);
    fn commit_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::mempool::CommitTransactionsRequest, sink: ::grpcio::UnarySink<super::mempool::CommitTransactionsResponse>);
    fn health_check(&mut self, ctx: ::grpcio::RpcContext, req: super::mempool::HealthCheckRequest, sink: ::grpcio::UnarySink<super::mempool::HealthCheckResponse>);
}

pub fn create_mempool<S: Mempool + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MEMPOOL_ADD_TRANSACTION_WITH_VALIDATION, move |ctx, req, resp| {
        instance.add_transaction_with_validation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MEMPOOL_GET_BLOCK, move |ctx, req, resp| {
        instance.get_block(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MEMPOOL_COMMIT_TRANSACTIONS, move |ctx, req, resp| {
        instance.commit_transactions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MEMPOOL_HEALTH_CHECK, move |ctx, req, resp| {
        instance.health_check(ctx, req, resp)
    });
    builder.build()
}
