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


pub trait SecretServiceClientTrait : Clone + Send + Sync {

    fn generate_key(&self, req: &super::secret_service::GenerateKeyRequest) -> ::grpcio::Result<super::secret_service::GenerateKeyResponse> {
        unimplemented!();
    }
    fn generate_key_async(&self, req: &super::secret_service::GenerateKeyRequest) -> ::grpcio::Result<Box<Future<Item=super::secret_service::GenerateKeyResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }

    fn get_public_key(&self, req: &super::secret_service::PublicKeyRequest) -> ::grpcio::Result<super::secret_service::PublicKeyResponse> {
        unimplemented!();
    }
    fn get_public_key_async(&self, req: &super::secret_service::PublicKeyRequest) -> ::grpcio::Result<Box<Future<Item=super::secret_service::PublicKeyResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }

    fn sign(&self, req: &super::secret_service::SignRequest) -> ::grpcio::Result<super::secret_service::SignResponse> {
        unimplemented!();
    }
    fn sign_async(&self, req: &super::secret_service::SignRequest) -> ::grpcio::Result<Box<Future<Item=super::secret_service::SignResponse, Error=::grpcio::Error> + Send>> {
        unimplemented!();
    }
}

impl SecretServiceClientTrait for super::secret_service_grpc::SecretServiceClient {

    fn generate_key(&self, req: &super::secret_service::GenerateKeyRequest) -> ::grpcio::Result<super::secret_service::GenerateKeyResponse> {
        self.generate_key(req)
    }
    fn generate_key_async(&self, req: &super::secret_service::GenerateKeyRequest) -> ::grpcio::Result<Box<Future<Item=super::secret_service::GenerateKeyResponse, Error=::grpcio::Error> + Send>> {
        match self.generate_key_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }

    fn get_public_key(&self, req: &super::secret_service::PublicKeyRequest) -> ::grpcio::Result<super::secret_service::PublicKeyResponse> {
        self.get_public_key(req)
    }
    fn get_public_key_async(&self, req: &super::secret_service::PublicKeyRequest) -> ::grpcio::Result<Box<Future<Item=super::secret_service::PublicKeyResponse, Error=::grpcio::Error> + Send>> {
        match self.get_public_key_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }

    fn sign(&self, req: &super::secret_service::SignRequest) -> ::grpcio::Result<super::secret_service::SignResponse> {
        self.sign(req)
    }
    fn sign_async(&self, req: &super::secret_service::SignRequest) -> ::grpcio::Result<Box<Future<Item=super::secret_service::SignResponse, Error=::grpcio::Error> + Send>> {
        match self.sign_async(req) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(e),
        }
    }
}
