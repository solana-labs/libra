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

const METHOD_EXECUTION_EXECUTE_BLOCK: ::grpcio::Method<super::execution::ExecuteBlockRequest, super::execution::ExecuteBlockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/execution.Execution/ExecuteBlock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_EXECUTION_COMMIT_BLOCK: ::grpcio::Method<super::execution::CommitBlockRequest, super::execution::CommitBlockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/execution.Execution/CommitBlock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_EXECUTION_EXECUTE_CHUNK: ::grpcio::Method<super::execution::ExecuteChunkRequest, super::execution::ExecuteChunkResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/execution.Execution/ExecuteChunk",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ExecutionClient {
    client: ::grpcio::Client,
}

impl ExecutionClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ExecutionClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn execute_block_opt(&self, req: &super::execution::ExecuteBlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::execution::ExecuteBlockResponse> {
        self.client.unary_call(&METHOD_EXECUTION_EXECUTE_BLOCK, req, opt)
    }

    pub fn execute_block(&self, req: &super::execution::ExecuteBlockRequest) -> ::grpcio::Result<super::execution::ExecuteBlockResponse> {
        self.execute_block_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_block_async_opt(&self, req: &super::execution::ExecuteBlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::execution::ExecuteBlockResponse>> {
        self.client.unary_call_async(&METHOD_EXECUTION_EXECUTE_BLOCK, req, opt)
    }

    pub fn execute_block_async(&self, req: &super::execution::ExecuteBlockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::execution::ExecuteBlockResponse>> {
        self.execute_block_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_block_opt(&self, req: &super::execution::CommitBlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::execution::CommitBlockResponse> {
        self.client.unary_call(&METHOD_EXECUTION_COMMIT_BLOCK, req, opt)
    }

    pub fn commit_block(&self, req: &super::execution::CommitBlockRequest) -> ::grpcio::Result<super::execution::CommitBlockResponse> {
        self.commit_block_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_block_async_opt(&self, req: &super::execution::CommitBlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::execution::CommitBlockResponse>> {
        self.client.unary_call_async(&METHOD_EXECUTION_COMMIT_BLOCK, req, opt)
    }

    pub fn commit_block_async(&self, req: &super::execution::CommitBlockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::execution::CommitBlockResponse>> {
        self.commit_block_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_chunk_opt(&self, req: &super::execution::ExecuteChunkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::execution::ExecuteChunkResponse> {
        self.client.unary_call(&METHOD_EXECUTION_EXECUTE_CHUNK, req, opt)
    }

    pub fn execute_chunk(&self, req: &super::execution::ExecuteChunkRequest) -> ::grpcio::Result<super::execution::ExecuteChunkResponse> {
        self.execute_chunk_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_chunk_async_opt(&self, req: &super::execution::ExecuteChunkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::execution::ExecuteChunkResponse>> {
        self.client.unary_call_async(&METHOD_EXECUTION_EXECUTE_CHUNK, req, opt)
    }

    pub fn execute_chunk_async(&self, req: &super::execution::ExecuteChunkRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::execution::ExecuteChunkResponse>> {
        self.execute_chunk_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Execution {
    fn execute_block(&mut self, ctx: ::grpcio::RpcContext, req: super::execution::ExecuteBlockRequest, sink: ::grpcio::UnarySink<super::execution::ExecuteBlockResponse>);
    fn commit_block(&mut self, ctx: ::grpcio::RpcContext, req: super::execution::CommitBlockRequest, sink: ::grpcio::UnarySink<super::execution::CommitBlockResponse>);
    fn execute_chunk(&mut self, ctx: ::grpcio::RpcContext, req: super::execution::ExecuteChunkRequest, sink: ::grpcio::UnarySink<super::execution::ExecuteChunkResponse>);
}

pub fn create_execution<S: Execution + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_EXECUTION_EXECUTE_BLOCK, move |ctx, req, resp| {
        instance.execute_block(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_EXECUTION_COMMIT_BLOCK, move |ctx, req, resp| {
        instance.commit_block(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_EXECUTION_EXECUTE_CHUNK, move |ctx, req, resp| {
        instance.execute_chunk(ctx, req, resp)
    });
    builder.build()
}
