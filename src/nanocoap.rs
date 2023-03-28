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

// todo: defining Class-entires with ResponseCodes useful? (e.g.: SUCCESS(CoapResponseCode) = ...)
pub enum CoapResponseCodeClass {
    // Response codes with 2.x
    Success = riot_sys::COAP_CLASS_SUCCESS as isize,

    // Response codes with 4.x
    // Can be response to any request
    // Should contain diagnostic data as payload
    ClientError = riot_sys::COAP_CLASS_CLIENT_FAILURE as isize,

    // Response codes with 5.x
    ServerError = riot_sys::COAP_CLASS_SERVER_FAILURE as isize,

    //a = riot_sys::COAP_CLASS_REQ as isize, todo: what semantics in nanocoap.h? needed here?
}

// todo: look up additional Response Codes outside of RFC 7252
pub enum CoapResponseCode {
    // Success (2.x)
    // 2.01, Response to POST and PUT
    Created = riot_sys::COAP_CODE_CREATED as isize,

    // 2.02, Response to DELETE and POST
    Deleted = riot_sys::COAP_CODE_DELETED as isize,

    // 2.03, todo: description
    Valid = riot_sys::COAP_CODE_VALID as isize,

    // 2.04, Response to POST and PUT
    Changed = riot_sys::COAP_CODE_CHANGED as isize,

    // 2.05, Response to GET, payload of packet has to have representation of the resource as payload
    Content = riot_sys::COAP_CODE_CONTENT as isize,

    // Client Error (4.x)
    // 4.00, Server can't or won't process request due to client error (e.g. syntax errors ...)
    BadRequest = riot_sys::COAP_CODE_BAD_REQUEST as isize,

    // 4.01, Client is not authorized for requested action; don't repeat request until authorization status with server changed
    Unauthorized = riot_sys::COAP_CODE_UNAUTHORIZED as isize,

    // 4.02, Server didn't recognize one or more options; don't repeat without modification of options
    BadOption = riot_sys::COAP_CODE_BAD_OPTION as isize,

    // 4.03, Client is not authenticated; don't repeat until authentication status changed
    Forbidden = riot_sys::COAP_CODE_FORBIDDEN as isize,

    // 4.04, Server didn't find or doesn't disclose if it has the requested resource
    NotFound = riot_sys::COAP_CODE_PATH_NOT_FOUND as isize,

    // 4.05, Requested method for the resource is not allowed or requested method is not recognized by the server; doesn't need payload
    MethodNotAllowed = riot_sys::COAP_CODE_METHOD_NOT_ALLOWED as isize,

    // 4.06, todo: description
    NotAcceptable = riot_sys::COAP_CODE_NOT_ACCEPTABLE as isize,

    // 4.12, Server doesn't meet preconditions from request header
    PreconditionFailed = riot_sys::COAP_CODE_PRECONDITION_FAILED as isize,

    // 4.13, Request size is too large for server; server can put the maximum length in the respoonse payload
    RequestEntityTooLarge = riot_sys::COAP_CODE_REQUEST_ENTITY_TOO_LARGE as isize,

    // 4.15, Server or resource doesn't support media format specified in request
    UnsupportedContentFormat = riot_sys::COAP_CODE_UNSUPPORTED_CONTENT_FORMAT as isize,

    // Internal Server Error (5.x)
    // 5.00, Generic error message, use if no other specific ServerError applies
    InternalServerError = riot_sys::COAP_CODE_INTERNAL_SERVER_ERROR as isize,

    // 5.01, Server can't fulfill request because it isn't supported (yet)
    NotImplemented = riot_sys::COAP_CODE_NOT_IMPLEMENTED as isize,

    // 5.02, Server acting as gateway or proxy encounters bad response from upstream server
    BadGateway = riot_sys::COAP_CODE_BAD_GATEWAY as isize,

    // 5.03, Server can't currently handle the request (usually temporarilly); Server sets Max-Age Option in response with number of seconds to retry after
    ServiceUnavailible = riot_sys::COAP_CODE_SERVICE_UNAVAILABLE as isize,

    // 5.04, Server acting as gateway or proxy didn't receive response from upstream server
    GatewayTimeout = riot_sys::COAP_CODE_GATEWAY_TIMEOUT as isize,

    // 5.05, Server can't or is unwilling to act as forward-proxy for URI specified in Proxy-Uri Option or Proxy-Scheme
    ProxyingNotSupported = riot_sys::COAP_CODE_PROXYING_NOT_SUPPORTED as isize,
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
        let ret: u32;
        unsafe {
            ret = riot_sys::inline::coap_get_type(self.pkt as *const riot_sys::inline::coap_pkt_t)
        }

        // todo: more elegant way to match? no_std alternative to TryFrom?
        // if elegant way is found, how to identify undefined types?
        match ret {
            riot_sys::COAP_TYPE_CON => Ok(CoapMessageType::CON),
            riot_sys::COAP_TYPE_NON => Ok(CoapMessageType::NON),
            riot_sys::COAP_TYPE_ACK => Ok(CoapMessageType::ACK),
            riot_sys::COAP_TYPE_RST => Ok(CoapMessageType::RST),
            _ => Err(CoapPacketError::TypeMatchError),
        }
    }

    // todo: Error possible or are undefined response codes caught by coap_pkt_init?
    pub fn get_message_code(
        &self,
    ) -> Result<(CoapResponseCodeClass, CoapResponseCode), CoapPacketError> {
        let ret: u32;
        unsafe {
            ret = riot_sys::inline::coap_get_code(self.pkt as *const riot_sys::inline::coap_pkt_t);
        }

        // todo: find elegant way to match
        // ret has value ResponseClass * 100 + CodeDetail
        match ret {
            _ => Err(CoapPacketError::CodeMatchError),
        }
    }

    // todo: Error possible or are undefined response codes caught by coap_pkt_init?
    pub fn get_message_code_class(&self) -> Result<CoapResponseCodeClass, CoapPacketError> {
        let ret: u32;
        unsafe {
            ret = riot_sys::inline::coap_get_code_class(
                self.pkt as *const riot_sys::inline::coap_pkt_t,
            );
        }

        // todo: find elegant way to match
        match ret {
            _ => Err(CoapPacketError::CodeMatchError),
        }
    }

    // todo: Error possible or are undefined response codes caught by coap_pkt_init?
    pub fn get_message_code_detail(&self) -> Result<CoapResponseCodeClass, CoapPacketError> {
        let ret: u32;
        unsafe {
            ret = riot_sys::inline::coap_get_code_detail(
                self.pkt as *const riot_sys::inline::coap_pkt_t,
            )
        }

        // todo: find elegant way to match
        match ret {
            _ => Err(CoapPacketError::CodeMatchError),
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

    /// todo: how to combine CodeDetail and CodeClass?
    /// todo: use enums for signature
    pub fn header_set_code(&self, code: u8) {
        unsafe {
            riot_sys::inline::coap_hdr_set_code(
                (*self.pkt).hdr as *mut riot_sys::inline::coap_hdr_t,
                code,
            );
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

