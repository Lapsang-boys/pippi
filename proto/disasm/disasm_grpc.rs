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

const METHOD_DISASSEMBLER_DISASSEMBLE: ::grpcio::Method<super::disasm::DisassembleRequest, super::disasm::DisassembleReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/disasm.Disassembler/Disassemble",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DisassemblerClient {
    client: ::grpcio::Client,
}

impl DisassemblerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DisassemblerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn disassemble_opt(&self, req: &super::disasm::DisassembleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::disasm::DisassembleReply> {
        self.client.unary_call(&METHOD_DISASSEMBLER_DISASSEMBLE, req, opt)
    }

    pub fn disassemble(&self, req: &super::disasm::DisassembleRequest) -> ::grpcio::Result<super::disasm::DisassembleReply> {
        self.disassemble_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disassemble_async_opt(&self, req: &super::disasm::DisassembleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::disasm::DisassembleReply>> {
        self.client.unary_call_async(&METHOD_DISASSEMBLER_DISASSEMBLE, req, opt)
    }

    pub fn disassemble_async(&self, req: &super::disasm::DisassembleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::disasm::DisassembleReply>> {
        self.disassemble_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Disassembler {
    fn disassemble(&mut self, ctx: ::grpcio::RpcContext, req: super::disasm::DisassembleRequest, sink: ::grpcio::UnarySink<super::disasm::DisassembleReply>);
}

pub fn create_disassembler<S: Disassembler + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DISASSEMBLER_DISASSEMBLE, move |ctx, req, resp| {
        instance.disassemble(ctx, req, resp)
    });
    builder.build()
}
