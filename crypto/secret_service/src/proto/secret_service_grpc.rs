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

const METHOD_SECRET_SERVICE_GENERATE_KEY: ::grpcio::Method<super::secret_service::GenerateKeyRequest, super::secret_service::GenerateKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/secret_service.SecretService/GenerateKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECRET_SERVICE_GET_PUBLIC_KEY: ::grpcio::Method<super::secret_service::PublicKeyRequest, super::secret_service::PublicKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/secret_service.SecretService/GetPublicKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SECRET_SERVICE_SIGN: ::grpcio::Method<super::secret_service::SignRequest, super::secret_service::SignResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/secret_service.SecretService/Sign",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SecretServiceClient {
    client: ::grpcio::Client,
}

impl SecretServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SecretServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn generate_key_opt(&self, req: &super::secret_service::GenerateKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_service::GenerateKeyResponse> {
        self.client.unary_call(&METHOD_SECRET_SERVICE_GENERATE_KEY, req, opt)
    }

    pub fn generate_key(&self, req: &super::secret_service::GenerateKeyRequest) -> ::grpcio::Result<super::secret_service::GenerateKeyResponse> {
        self.generate_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn generate_key_async_opt(&self, req: &super::secret_service::GenerateKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_service::GenerateKeyResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_SERVICE_GENERATE_KEY, req, opt)
    }

    pub fn generate_key_async(&self, req: &super::secret_service::GenerateKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_service::GenerateKeyResponse>> {
        self.generate_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_public_key_opt(&self, req: &super::secret_service::PublicKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_service::PublicKeyResponse> {
        self.client.unary_call(&METHOD_SECRET_SERVICE_GET_PUBLIC_KEY, req, opt)
    }

    pub fn get_public_key(&self, req: &super::secret_service::PublicKeyRequest) -> ::grpcio::Result<super::secret_service::PublicKeyResponse> {
        self.get_public_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_public_key_async_opt(&self, req: &super::secret_service::PublicKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_service::PublicKeyResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_SERVICE_GET_PUBLIC_KEY, req, opt)
    }

    pub fn get_public_key_async(&self, req: &super::secret_service::PublicKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_service::PublicKeyResponse>> {
        self.get_public_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_opt(&self, req: &super::secret_service::SignRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::secret_service::SignResponse> {
        self.client.unary_call(&METHOD_SECRET_SERVICE_SIGN, req, opt)
    }

    pub fn sign(&self, req: &super::secret_service::SignRequest) -> ::grpcio::Result<super::secret_service::SignResponse> {
        self.sign_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_async_opt(&self, req: &super::secret_service::SignRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_service::SignResponse>> {
        self.client.unary_call_async(&METHOD_SECRET_SERVICE_SIGN, req, opt)
    }

    pub fn sign_async(&self, req: &super::secret_service::SignRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::secret_service::SignResponse>> {
        self.sign_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SecretService {
    fn generate_key(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_service::GenerateKeyRequest, sink: ::grpcio::UnarySink<super::secret_service::GenerateKeyResponse>);
    fn get_public_key(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_service::PublicKeyRequest, sink: ::grpcio::UnarySink<super::secret_service::PublicKeyResponse>);
    fn sign(&mut self, ctx: ::grpcio::RpcContext, req: super::secret_service::SignRequest, sink: ::grpcio::UnarySink<super::secret_service::SignResponse>);
}

pub fn create_secret_service<S: SecretService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_SERVICE_GENERATE_KEY, move |ctx, req, resp| {
        instance.generate_key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_SERVICE_GET_PUBLIC_KEY, move |ctx, req, resp| {
        instance.get_public_key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SECRET_SERVICE_SIGN, move |ctx, req, resp| {
        instance.sign(ctx, req, resp)
    });
    builder.build()
}
