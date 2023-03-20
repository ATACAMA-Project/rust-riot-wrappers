//! Wrapper containing structs and functions from nanocoap.h
//!
use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use riot_wrappers::{println, riot_sys, riot_sys::libc};

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

pub enum CoapMessageType {
    CON = riot_sys::COAP_TYPE_CON as isize,
    NON = riot_sys::COAP_TYPE_NON as isize,
    ACK = riot_sys::COAP_TYPE_ACK as isize,
    RST = riot_sys::COAP_TYPE_RST as isize,
}

///todo: fill
pub enum CoapVersion {}

//todo: more elegant naming
pub enum CoapAcceptFormat {
    Accept,
    NonAccept,
    // Add None (COAP_FORMAT_NONE) as error in CoapPacketError?
    None,
}

/// todo: remove Header struct, implement the methods in CoapPacket
/// {
pub struct CoapHeader {
    pub hdr: *mut riot_sys::coap_hdr_t,
}

impl CoapHeader {
    /// user must ensure hdr can hold the header and the full token!
    /// @note Wraps riot_sys::coap_build_hdr
    pub fn build_hdr(
        type_: u32,
        token: &[u8],
        code: u32,
        id: u16,
    ) -> Result<(CoapHeader, isize), CoapPacketError> {
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
            if ret < 0 {
                Err(CoapPacketError::HeaderBuildError)
            } else {
                Ok((CoapHeader { hdr }, ret as isize))
            }
        }
    }
}
/// }

/// justified to have their own structs? {
pub struct CoapOptposT {
    pub coap_optpos_t_C: riot_sys::coap_optpos_t,
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
/// }

pub struct CoapPacket {
    pkt: *mut riot_sys::coap_pkt_t
}

/// WIP
pub enum CoapPacketError {
    HeaderBuildError,
    ParseError,
    ReplyError,
    SmallBufferError,
    PayloadWriteError,
    TreeHandleError,
    TypeMatchError,
}

impl CoapPacket {
    ///@note: Wraps riot_sys::coap_pkt_init
    /// combine to new() function with coap_build_hdr
    ///     --> come up with buffer solution
    pub fn new_from_header(buf: &[u8], hdr_len: usize) -> CoapPacket {
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

            if ret < 0 {
                Err(CoapPacketError::ParseError)
            } else {
                Ok(CoapPacket { pkt })
            }
        }
    }

    /// todo: make pretty, fill and return CoapVersionNumber enum
    pub fn get_version(&self) -> u32 {
        unsafe {
            riot_sys::inline::coap_get_ver(self.pkt as *const riot_sys::inline::coap_pkt_t)
        }
    }

    pub fn get_id(&self) -> u32 {
        unsafe {
            riot_sys::inline::coap_get_id(self.pkt as *const riot_sys::inline::coap_pkt_t)
        }
    }

    pub fn get_type(&self) -> Result<CoapMessageType, CoapPacketError> {
        unsafe {
            match riot_sys::inline::coap_get_type(self.pkt as *const riot_sys::inline::coap_pkt_t) {
                riot_sys::COAP_TYPE_CON => Ok(CoapMessageType::CON),
                riot_sys::COAP_TYPE_NON => Ok(CoapMessageType::NON),
                riot_sys::COAP_TYPE_ACK => Ok(CoapMessageType::ACK),
                riot_sys::COAP_TYPE_RST => Ok(CoapMessageType::RST),
                _ => Err(CoapPacketError::TypeMatchError),
            }
        }
    }

    /// Response-codes as enum?
    pub fn get_message_code_raw(&self) -> u32 {
        unsafe {
            riot_sys::inline::coap_get_code_raw(
                self.pkt as *const riot_sys::inline::coap_pkt_t,
            )
        }
    }

    /// Response-codes as enum?
    pub fn get_message_code(&self) -> u32 {
        unsafe {
            riot_sys::inline::coap_get_code(
                self.pkt as *const riot_sys::inline::coap_pkt_t,
            )
        }
    }

    /// Response-codes as enum?
    pub fn get_message_code_detail(&self) -> u32 {
        unsafe {
            riot_sys::inline::coap_get_code_detail(
                self.pkt as *const riot_sys::inline::coap_pkt_t,
            )
        }
    }

    /// Semantics of this function? Maybe content type as enum?
    /// todo: catch COAP_FORMAT_NONE
    pub fn get_content_type(&self) -> u32 {
        unsafe {
            riot_sys:: coap_get_content_type(self.pkt)
        }
    }

    //todo: clarify and rework enum
    pub fn get_accept(&self) -> CoapAcceptFormat {
        unsafe {
            match riot_sys::coap_get_accept(self.pkt) {
                //correct constant?
                riot_sys::COAP_OPT_ACCEPT => CoapAcceptFormat::Accept,
                //no non-accept?
                _ => CoapAcceptFormat::None,
            }
        }
    }

    /// Response-codes as enum?
    pub fn header_set_code(&self, code: u8) {
        unsafe {
            riot_sys::inline::coap_hdr_set_code((*self.pkt).hdr as *mut riot_sys::inline::coap_hdr_t, code);
        }
    }

    /// what exacly do types represent?
    /// todo: catch COAP_FORMAT_NONE
    pub fn header_set_type(&self, type_: u32) {
        unsafe {
            riot_sys::inline::coap_hdr_set_type(
                (*self.pkt).hdr as *mut riot_sys::inline::coap_hdr_t,
                type_ as libc::c_uint,
            );
        }
    }
}

