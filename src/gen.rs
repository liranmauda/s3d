//! This file is generated by build.rs. DO NOT EDIT.
#![allow(unused)]

use crate::proto::*;
use aws_smithy_http::byte_stream::ByteStream;
use aws_smithy_xml::{decode::ScopedDecoder, encode::ScopeWriter};
use hyper::{body::Bytes, header::HeaderValue, Body, StatusCode};

include!(concat!(env!("OUT_DIR"), "/s3.base.rs"));
include!(concat!(env!("OUT_DIR"), "/s3.enums.rs"));

// pub mod smithy {
//     use super::*;
//     include!(concat!(env!("OUT_DIR"), "/s3.smithy.rs"));
// }
// pub mod server {
//     use super::*;
//     include!(concat!(env!("OUT_DIR"), "/s3.server.rs"));
// }
// pub mod xml {
//     use super::*;
//     include!(concat!(env!("OUT_DIR"), "/s3.xml.rs"));
// }