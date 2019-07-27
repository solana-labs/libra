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

const METHOD_NODE_DEBUG_INTERFACE_GET_NODE_DETAILS: ::grpcio::Method<super::node_debug_interface::GetNodeDetailsRequest, super::node_debug_interface::GetNodeDetailsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debug.NodeDebugInterface/GetNodeDetails",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NODE_DEBUG_INTERFACE_DUMP_JEMALLOC_HEAP_PROFILE: ::grpcio::Method<super::node_debug_interface::DumpJemallocHeapProfileRequest, super::node_debug_interface::DumpJemallocHeapProfileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debug.NodeDebugInterface/DumpJemallocHeapProfile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct NodeDebugInterfaceClient {
    client: ::grpcio::Client,
}

impl NodeDebugInterfaceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NodeDebugInterfaceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_node_details_opt(&self, req: &super::node_debug_interface::GetNodeDetailsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::node_debug_interface::GetNodeDetailsResponse> {
        self.client.unary_call(&METHOD_NODE_DEBUG_INTERFACE_GET_NODE_DETAILS, req, opt)
    }

    pub fn get_node_details(&self, req: &super::node_debug_interface::GetNodeDetailsRequest) -> ::grpcio::Result<super::node_debug_interface::GetNodeDetailsResponse> {
        self.get_node_details_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_details_async_opt(&self, req: &super::node_debug_interface::GetNodeDetailsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node_debug_interface::GetNodeDetailsResponse>> {
        self.client.unary_call_async(&METHOD_NODE_DEBUG_INTERFACE_GET_NODE_DETAILS, req, opt)
    }

    pub fn get_node_details_async(&self, req: &super::node_debug_interface::GetNodeDetailsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node_debug_interface::GetNodeDetailsResponse>> {
        self.get_node_details_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn dump_jemalloc_heap_profile_opt(&self, req: &super::node_debug_interface::DumpJemallocHeapProfileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::node_debug_interface::DumpJemallocHeapProfileResponse> {
        self.client.unary_call(&METHOD_NODE_DEBUG_INTERFACE_DUMP_JEMALLOC_HEAP_PROFILE, req, opt)
    }

    pub fn dump_jemalloc_heap_profile(&self, req: &super::node_debug_interface::DumpJemallocHeapProfileRequest) -> ::grpcio::Result<super::node_debug_interface::DumpJemallocHeapProfileResponse> {
        self.dump_jemalloc_heap_profile_opt(req, ::grpcio::CallOption::default())
    }

    pub fn dump_jemalloc_heap_profile_async_opt(&self, req: &super::node_debug_interface::DumpJemallocHeapProfileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node_debug_interface::DumpJemallocHeapProfileResponse>> {
        self.client.unary_call_async(&METHOD_NODE_DEBUG_INTERFACE_DUMP_JEMALLOC_HEAP_PROFILE, req, opt)
    }

    pub fn dump_jemalloc_heap_profile_async(&self, req: &super::node_debug_interface::DumpJemallocHeapProfileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::node_debug_interface::DumpJemallocHeapProfileResponse>> {
        self.dump_jemalloc_heap_profile_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NodeDebugInterface {
    fn get_node_details(&mut self, ctx: ::grpcio::RpcContext, req: super::node_debug_interface::GetNodeDetailsRequest, sink: ::grpcio::UnarySink<super::node_debug_interface::GetNodeDetailsResponse>);
    fn dump_jemalloc_heap_profile(&mut self, ctx: ::grpcio::RpcContext, req: super::node_debug_interface::DumpJemallocHeapProfileRequest, sink: ::grpcio::UnarySink<super::node_debug_interface::DumpJemallocHeapProfileResponse>);
}

pub fn create_node_debug_interface<S: NodeDebugInterface + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NODE_DEBUG_INTERFACE_GET_NODE_DETAILS, move |ctx, req, resp| {
        instance.get_node_details(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NODE_DEBUG_INTERFACE_DUMP_JEMALLOC_HEAP_PROFILE, move |ctx, req, resp| {
        instance.dump_jemalloc_heap_profile(ctx, req, resp)
    });
    builder.build()
}
