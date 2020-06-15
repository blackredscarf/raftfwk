// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
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
//! Generated file from `mpc.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct MpcReply {
    // message fields
    pub code: MpcReplyCode,
    pub msg: ::std::string::String,
    pub context: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MpcReply {
    fn default() -> &'a MpcReply {
        <MpcReply as ::protobuf::Message>::default_instance()
    }
}

impl MpcReply {
    pub fn new() -> MpcReply {
        ::std::default::Default::default()
    }

    // .mpc.MpcReplyCode code = 1;


    pub fn get_code(&self) -> MpcReplyCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = MpcReplyCode::Ok;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: MpcReplyCode) {
        self.code = v;
    }

    // string msg = 2;


    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    // bytes context = 3;


    pub fn get_context(&self) -> &[u8] {
        &self.context
    }
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.context = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.context
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.context, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for MpcReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.context)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != MpcReplyCode::Ok {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        if !self.context.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.context);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != MpcReplyCode::Ok {
            os.write_enum(1, self.code.value())?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        if !self.context.is_empty() {
            os.write_bytes(3, &self.context)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MpcReply {
        MpcReply::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MpcReplyCode>>(
                    "code",
                    |m: &MpcReply| { &m.code },
                    |m: &mut MpcReply| { &mut m.code },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    |m: &MpcReply| { &m.msg },
                    |m: &mut MpcReply| { &mut m.msg },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context",
                    |m: &MpcReply| { &m.context },
                    |m: &mut MpcReply| { &mut m.context },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<MpcReply>(
                    "MpcReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static MpcReply {
        static mut instance: ::protobuf::lazy::Lazy<MpcReply> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(MpcReply::new)
        }
    }
}

impl ::protobuf::Clear for MpcReply {
    fn clear(&mut self) {
        self.code = MpcReplyCode::Ok;
        self.msg.clear();
        self.context.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MpcReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MpcReply {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MpcReplyCode {
    Ok = 0,
    ERR = 1,
}

impl ::protobuf::ProtobufEnum for MpcReplyCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MpcReplyCode> {
        match value {
            0 => ::std::option::Option::Some(MpcReplyCode::Ok),
            1 => ::std::option::Option::Some(MpcReplyCode::ERR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MpcReplyCode] = &[
            MpcReplyCode::Ok,
            MpcReplyCode::ERR,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new_pb_name::<MpcReplyCode>("MpcReplyCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MpcReplyCode {
}

impl ::std::default::Default for MpcReplyCode {
    fn default() -> Self {
        MpcReplyCode::Ok
    }
}

impl ::protobuf::reflect::ProtobufValue for MpcReplyCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tmpc.proto\x12\x03mpc\x1a\nmmsg.proto\"]\n\x08MpcReply\x12%\n\x04code\
    \x18\x01\x20\x01(\x0e2\x11.mpc.MpcReplyCodeR\x04code\x12\x10\n\x03msg\
    \x18\x02\x20\x01(\tR\x03msg\x12\x18\n\x07context\x18\x03\x20\x01(\x0cR\
    \x07context*\x1f\n\x0cMpcReplyCode\x12\x06\n\x02Ok\x10\0\x12\x07\n\x03ER\
    R\x10\x012-\n\x03Mpc\x12&\n\x04send\x12\r.mmsg.Message\x1a\r.mpc.MpcRepl\
    y\"\0J\x9d\x03\n\x06\x12\x04\x01\0\x13\x03\n\x08\n\x01\x0c\x12\x03\x01\0\
    \x12\n\x08\n\x01\x02\x12\x03\x03\0\x0c\n\t\n\x02\x03\0\x12\x03\x04\0\x14\
    \n\n\n\x02\x06\0\x12\x04\x06\0\x08\x01\n\n\n\x03\x06\0\x01\x12\x03\x06\
    \x08\x0b\n\x0b\n\x04\x06\0\x02\0\x12\x03\x07\x02.\n\x0c\n\x05\x06\0\x02\
    \0\x01\x12\x03\x07\x06\n\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x07\x0b\x17\
    \n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x07\"*\n\n\n\x02\x04\0\x12\x04\n\0\
    \x0e\x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x10\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x0b\x04\x1a\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0b\x04\x10\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\x11\x15\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x0b\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x04\x13\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0c\x04\n\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x0c\x0b\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c\x11\
    \x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\r\x04\x16\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\r\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\n\x11\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\r\x14\x15\n\n\n\x02\x05\0\x12\x04\
    \x10\0\x13\x03\n\n\n\x03\x05\0\x01\x12\x03\x10\x05\x11\n\x0b\n\x04\x05\0\
    \x02\0\x12\x03\x11\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x11\x04\
    \x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x11\t\n\n\x0b\n\x04\x05\0\x02\
    \x01\x12\x03\x12\x04\x0c\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x12\x04\
    \x07\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x12\n\x0bb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
