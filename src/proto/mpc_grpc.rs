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

const METHOD_MPC_SEND: ::grpcio::Method<super::mmsg::Message, super::mpc::MpcReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mpc.Mpc/send",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MpcClient {
    client: ::grpcio::Client,
}

impl MpcClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MpcClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_opt(&self, req: &super::mmsg::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mpc::MpcReply> {
        self.client.unary_call(&METHOD_MPC_SEND, req, opt)
    }

    pub fn send(&self, req: &super::mmsg::Message) -> ::grpcio::Result<super::mpc::MpcReply> {
        self.send_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_async_opt(&self, req: &super::mmsg::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpc::MpcReply>> {
        self.client.unary_call_async(&METHOD_MPC_SEND, req, opt)
    }

    pub fn send_async(&self, req: &super::mmsg::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpc::MpcReply>> {
        self.send_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures_01::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Mpc {
    fn send(&mut self, ctx: ::grpcio::RpcContext, req: super::mmsg::Message, sink: ::grpcio::UnarySink<super::mpc::MpcReply>);
}

pub fn create_mpc<S: Mpc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_MPC_SEND, move |ctx, req, resp| {
        instance.send(ctx, req, resp)
    });
    builder.build()
}
