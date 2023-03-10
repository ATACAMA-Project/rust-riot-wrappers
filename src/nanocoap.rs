//! Wrapper containing structs and functions from nanocoap.h
//!
use crate::helpers::SliceToCStr;
use crate::nanocoap::CoapPacketError::HeaderBuildError;
use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use riot_sys::coap_hdr_t;
use riot_wrappers::riot_sys::inline::{uint16_t, uint8_t};
use riot_wrappers::riot_sys::{coap_optpos_t, coap_pkt_t, iolist, libc, u_int, uint};
use riot_wrappers::{println, riot_sys};

//! Relevant structs and enums from nanocoap.h
//todo: add remaining structs and enums

pub enum CoapMethodFlags {
    GET = riot_sys::COAP_METHOD_GET as isize,
    PUT = riot_sys::COAP_METHOD_PUT as isize,
    POST = riot_sys::COAP_METHOD_POST as isize,
    DELETE = riot_sys::COAP_METHOD_DELETE as isize,
    FETCH = riot_sys::COAP_METHOD_FETCH as isize,
    PATCH = riot_sys::COAP_METHOD_PATCH as isize,
    IPATCH = riot_sys::COAP_METHOD_IPATCH as isize,
}

pub struct CoapHdrT {
    pub coap_hdr_t_C: coap_hdr_t,
}

pub struct CoapOptposT {
    pub coap_optpos_t_C: coap_optpos_t,
}

pub struct CoapResource {
    resource: *mut riot_sys::coap_resource_t
}

pub struct CoapHandler {
    handler: *mut libc::c_void
}

pub struct CoapRequestContext {
    ctx: *mut riot_sys::coap_request_ctx_t
}

impl CoapHdrT {
    pub fn new(ver_t_tkl: u8, code: u8, id: u16) -> CoapHdrT {
        CoapHdrT {
            coap_hdr_t_C: coap_hdr_t {
                ver_t_tkl,
                code,
                id,
            },
        }
    }

    /// user must ensure hdr can hold the header and the full token!
    /// @note Wraps riot_sys::coap_build_hdr
    pub fn build_hdr(
        type_: u32,
        token: &[u8],
        code: u32,
        id: u16,
    ) -> Result<(*mut riot_sys::coap_hdr_t, riot_sys::ssize_t), CoapPacketError> {
        unsafe {
            let hdr: *mut riot_sys::coap_hdr_t = MaybeUninit::uninit().assume_init_mut();
            let ret = riot_sys::coap_build_hdr(
                hdr,
                type_ as libc::c_uint,
                token.as_ptr() as *mut u8,
                token.len() as riot_sys::size_t,
                code as libc::c_uint,
                id,
            );
            return if ret <= 0 {
                Err(HeaderBuildError)
            } else {
                Ok((hdr, ret))
            };
        }
    }
}


impl CoapOptposT {
    pub fn new(opt_num: u16, offset: u16) -> CoapOptposT {
        CoapOptposT {
            coap_optpos_t_C: coap_optpos_t { opt_num, offset },
        }
    }
}


pub struct CoapPacket {
    pkt: *mut riot_sys::coap_pkt_t,
}

enum CoapPacketError {
    ParseError,
    HeaderBuildError,
}

impl CoapPacket {
    ///@note: Wraps riot_sys::coap_pkt_init
    pub fn new_with_header(buf: &[u8], hdr_len: usize) -> CoapPacket {
        unsafe {
            let pkt: *mut riot_sys::coap_pkt_t = MaybeUninit::uninit().assume_init_mut();
            riot_sys::coap_pkt_init(
                pkt,
                buf.as_ptr() as *mut u8,
                buf.len() as riot_sys::size_t,
                hdr_len as riot_sys::size_t,
            );

            CoapPacket { pkt }
        }
    }


    ///@note: Wraps riot_sys::coap_parse
    /// Parse a CoAP PDU
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer containing a received CoAP-Message
    ///
    /// # Return
    ///
    /// Returns Ok with the parsed CoapPacket and Err with an error message otherwise
    ///
    pub fn read_packet(mut buf: &[u8]) -> Result<CoapPacket, CoapPacketError> {
        unsafe {
            let pkt: *mut riot_sys::coap_pkt_t = MaybeUninit::uninit().assume_init_mut();
            let ret =
                riot_sys::coap_parse(pkt, buf.as_ptr() as *mut u8, buf.len() as riot_sys::size_t);

            return if ret < 0 {
                return Err(CoapPacketError::ParseError);
            } else {
                Ok(CoapPacket { pkt })
            };
        }
    }
}

//! Defining relevant methods for coap_resource_t
impl CoapResource {
    //todo: add remaining methods

    //! Beispiel, wie eine Methode aussehen kann
    //todo: fix pointer issue
    fn new(path: &str, flags: CoapMethodFlags, handler: CoapHandler, ctx: CoapRequestContext) -> CoapResource {
             CoapResource {
            resource: riot_sys::coap_resource_t {
                path: path.as_bytes().to_cstr().unwrap().as_ptr(),
                methods: flags as riot_sys::coap_method_flags_t,
                handler: handler.handler,
                context: ctx.ctx
            }
        };
    }
}
