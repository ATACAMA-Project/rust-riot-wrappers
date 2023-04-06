//! Wrapper containing structs and functions from nanocoap.h
//!
use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use riot_wrappers::{println, riot_sys, riot_sys::libc};

//! Relevant structs and enums from nanocoap.h

// fallback if no size for a message-buffer is given
const COAP_DEFAULT_BUFFER_SIZE: usize = 1024;

/// WIP
pub enum CoapPacketError {
    HeaderBuildError,
    ParseError,
    ReplyError,
    SmallBufferError,
    PayloadWriteError,
    TreeHandleError,
    TypeMatchError,
    CodeMatchError,
    MethodMatchError,
    ClassDetailCombinationError,
    MallocError,
    HandlingError,
    NoRequestError,
}

pub enum CoapMethod {
    Get,
    Put,
    Post,
    Delete,
    Fetch,
    Patch,
    IPatch,
}

impl CoapMethod {
    pub fn from_c(input: u32) -> Result<Self, CoapPacketError> {
        match input {
            riot_sys::COAP_METHOD_GET => Ok(Self::Get),
            riot_sys::COAP_METHOD_PUT => Ok(Self::Put),
            riot_sys::COAP_METHOD_POST => Ok(Self::Post),
            riot_sys::COAP_METHOD_DELETE => Ok(Self::Delete),
            riot_sys::COAP_METHOD_FETCH => Ok(Self::Fetch),
            riot_sys::COAP_METHOD_PATCH => Ok(Self::Patch),
            riot_sys::COAP_METHOD_IPATCH => Ok(Self::IPatch),
            _ => Err(CoapPacketError::MethodMatchError),
        }
    }

    pub fn to_c(self) -> u32 {
        match self {
            Self::Get => riot_sys::COAP_METHOD_GET,
            Self::Put => riot_sys::COAP_METHOD_PUT,
            Self::Post => riot_sys::COAP_METHOD_POST,
            Self::Delete => riot_sys::COAP_METHOD_DELETE,
            Self::Fetch => riot_sys::COAP_METHOD_FETCH,
            Self::Patch => riot_sys::COAP_METHOD_PATCH,
            Self::IPatch => riot_sys::COAP_METHOD_IPATCH,
        }
    }
}

pub enum CoapMessageType {
    Con,
    Non,
    Ack,
    Rst,
}

impl CoapMessageType {
    pub fn from_c(input: u32) -> Result<Self, CoapPacketError> {
        match input {
            riot_sys::COAP_TYPE_CON => Ok(Self::Con),
            riot_sys::COAP_TYPE_NON => Ok(Self::Non),
            riot_sys::COAP_TYPE_ACK => Ok(Self::Ack),
            riot_sys::COAP_TYPE_RST => Ok(Self::Rst),
            _ => Err(CoapPacketError::TypeMatchError),
        }
    }

    pub fn to_c(self) -> u32 {
        match self {
            Self::Con => riot_sys::COAP_TYPE_CON,
            Self::Non => riot_sys::COAP_TYPE_NON,
            Self::Ack => riot_sys::COAP_TYPE_ACK,
            Self::Rst => riot_sys::COAP_TYPE_RST,
        }
    }
}

pub enum CoapCodeClass {
    // Requests 0.x
    Request,

    // Response codes with 2.x
    Success,

    // Response codes with 4.x
    // Can be response to any request
    // Should contain diagnostic data as payload
    ClientError,

    // Response codes with 5.x
    ServerError,
}

impl CoapCodeClass {
    pub fn from_c(input: u32) -> Result<Self, CoapPacketError> {
        match input {
            riot_sys::COAP_CLASS_REQ => Ok(Self::Request),
            riot_sys::COAP_CLASS_SUCCESS => Ok(Self::Success),
            riot_sys::COAP_CLASS_CLIENT_FAILURE => Ok(Self::ClientError),
            riot_sys::COAP_CLASS_SERVER_FAILURE => Ok(Self::ServerError),
            _ => Err(CoapPacketError::CodeMatchError),
        }
    }

    pub fn to_c(self) -> u32 {
        match self {
            Self::Request => riot_sys::COAP_CLASS_REQ,
            Self::Success => riot_sys::COAP_CLASS_SUCCESS,
            Self::ClientError => riot_sys::COAP_CLASS_CLIENT_FAILURE,
            Self::ServerError => riot_sys::COAP_CLASS_SERVER_FAILURE,
        }
    }
}

pub enum CoapCode {
    // Success (2.x)
    // 2.01, Response to POST and PUT
    Created,

    // 2.02, Response to DELETE and POST
    Deleted,

    // 2.03, todo: description
    Valid,

    // 2.04, Response to POST and PUT
    Changed,

    // 2.05, Response to GET, payload of packet has to have representation of the resource as payload
    Content,

    // 2.31, Transfer of block was successful, server encourages sending further blocks; outcome of block-wise request can't be determined yet
    // Only needed for Blockwise Transfer (RFC 7959)!
    Continue,

    // Client Error (4.x)
    // 4.00, Server can't or won't process request due to client error (e.g. syntax errors ...)
    BadRequest,

    // 4.01, Client is not authorized for requested action; don't repeat request until authorization status with server changed
    Unauthorized,

    // 4.02, Server didn't recognize one or more options; don't repeat without modification of options
    BadOption,

    // 4.03, Client is not authenticated; don't repeat until authentication status changed
    Forbidden,

    // 4.04, Server didn't find or doesn't disclose if it has the requested resource
    NotFound,

    // 4.05, Requested method for the resource is not allowed or requested method is not recognized by the server; doesn't need payload
    MethodNotAllowed,

    // 4.06, todo: description
    NotAcceptable,

    // 4.08, Server has not yet received all blocks of the request body it needs to proceed; Client either has not yet sent all blocks, sent them out of order or server has already discarded them
    // Only needed for Blockwise Transfer (RFC 7959)!
    RequestEntityIncomplete,

    // 4.09, todo: description
    Conflict,

    // 4.12, Server doesn't meet preconditions from request header
    PreconditionFailed,

    // 4.13, Request size is too large for server; server can put the maximum length in the respoonse payload
    // With RFC 7959: Can be sent by server at any time during Block1-transfer if it currently doesn't have enough resources to store all blocks
    RequestEntityTooLarge,

    // 4.15, Server or resource doesn't support media format specified in request
    UnsupportedContentFormat,

    // 4.22, todo: description
    UnprocessableEntity,

    // 4.29, todo: description
    TooManyRequests,

    // Internal Server Error (5.x)
    // 5.00, Generic error message, use if no other specific ServerError applies
    InternalServerError,

    // 5.01, Server can't fulfill request because it isn't supported (yet)
    NotImplemented,

    // 5.02, Server acting as gateway or proxy encounters bad response from upstream server
    BadGateway,

    // 5.03, Server can't currently handle the request (usually temporarily); Server sets Max-Age Option in response with number of seconds to retry after
    ServiceUnavailible,

    // 5.04, Server acting as gateway or proxy didn't receive response from upstream server
    GatewayTimeout,

    // 5.05, Server can't or is unwilling to act as forward-proxy for URI specified in Proxy-Uri Option or Proxy-Scheme
    ProxyingNotSupported,
}

impl CoapCode {
    pub fn from_c(input: u32) -> Result<CoapCode, CoapPacketError> {
        match input {
            riot_sys::COAP_CODE_CREATED => Ok(Self::Created),
            riot_sys::COAP_CODE_DELETED => Ok(Self::Deleted),
            riot_sys::COAP_CODE_VALID => Ok(Self::Valid),
            riot_sys::COAP_CODE_CHANGED => Ok(Self::Changed),
            riot_sys::COAP_CODE_CONTENT => Ok(Self::Content),
            riot_sys::COAP_CODE_CONTINUE => Ok(Self::Continue),
            riot_sys::COAP_CODE_BAD_REQUEST => Ok(Self::BadRequest),
            riot_sys::COAP_CODE_UNAUTHORIZED => Ok(Self::Unauthorized),
            riot_sys::COAP_CODE_BAD_OPTION => Ok(Self::BadOption),
            riot_sys::COAP_CODE_FORBIDDEN => Ok(Self::Forbidden),
            riot_sys::COAP_CODE_PATH_NOT_FOUND => Ok(Self::NotFound),
            riot_sys::COAP_CODE_METHOD_NOT_ALLOWED => Ok(Self::MethodNotAllowed),
            riot_sys::COAP_CODE_NOT_ACCEPTABLE => Ok(Self::NotAcceptable),
            riot_sys::COAP_CODE_REQUEST_ENTITY_INCOMPLETE => Ok(Self::RequestEntityIncomplete),
            riot_sys::COAP_CODE_CONFLICT => Ok(Self::Conflict),
            riot_sys::COAP_CODE_PRECONDITION_FAILED => Ok(Self::PreconditionFailed),
            riot_sys::COAP_CODE_REQUEST_ENTITY_TOO_LARGE => Ok(Self::RequestEntityTooLarge),
            riot_sys::COAP_CODE_UNSUPPORTED_CONTENT_FORMAT => Ok(Self::UnsupportedContentFormat),
            riot_sys::COAP_CODE_UNPROCESSABLE_ENTITY => Ok(Self::UnprocessableEntity),
            riot_sys::COAP_CODE_TOO_MANY_REQUESTS => Ok(Self::TooManyRequests),
            riot_sys::COAP_CODE_INTERNAL_SERVER_ERROR => Ok(Self::InternalServerError),
            riot_sys::COAP_CODE_NOT_IMPLEMENTED => Ok(Self::NotImplemented),
            riot_sys::COAP_CODE_BAD_GATEWAY => Ok(Self::BadGateway),
            riot_sys::COAP_CODE_SERVICE_UNAVAILABLE => Ok(Self::ServiceUnavailible),
            riot_sys::COAP_CODE_GATEWAY_TIMEOUT => Ok(Self::GatewayTimeout),
            riot_sys::COAP_CODE_PROXYING_NOT_SUPPORTED => Ok(Self::ProxyingNotSupported),
            _ => Err(CoapPacketError::CodeMatchError),
        }
    }

    pub fn to_c(self) -> u32 {
        match self {
            Self::Created => riot_sys::COAP_CODE_CREATED,
            Self::Deleted => riot_sys::COAP_CODE_DELETED,
            Self::Valid => riot_sys::COAP_CODE_VALID,
            Self::Changed => riot_sys::COAP_CODE_CHANGED,
            Self::Content => riot_sys::COAP_CODE_CONTENT,
            Self::Continue => riot_sys::COAP_CODE_CONTINUE,
            Self::BadRequest => riot_sys::COAP_CODE_BAD_REQUEST,
            Self::Unauthorized => riot_sys::COAP_CODE_UNAUTHORIZED,
            Self::BadOption => riot_sys::COAP_CODE_BAD_OPTION,
            Self::Forbidden => riot_sys::COAP_CODE_FORBIDDEN,
            Self::NotFound => riot_sys::COAP_CODE_PATH_NOT_FOUND,
            Self::MethodNotAllowed => riot_sys::COAP_CODE_METHOD_NOT_ALLOWED,
            Self::NotAcceptable => riot_sys::COAP_CODE_NOT_ACCEPTABLE,
            Self::RequestEntityIncomplete => riot_sys::COAP_CODE_REQUEST_ENTITY_INCOMPLETE,
            Self::Conflict => riot_sys::COAP_CODE_CONFLICT,
            Self::PreconditionFailed => riot_sys::COAP_CODE_PRECONDITION_FAILED,
            Self::RequestEntityTooLarge => riot_sys::COAP_CODE_REQUEST_ENTITY_TOO_LARGE,
            Self::UnsupportedContentFormat => riot_sys::COAP_CODE_UNSUPPORTED_CONTENT_FORMAT,
            Self::UnprocessableEntity => riot_sys::COAP_CODE_UNPROCESSABLE_ENTITY,
            Self::TooManyRequests => riot_sys::COAP_CODE_TOO_MANY_REQUESTS,
            Self::InternalServerError => riot_sys::COAP_CODE_INTERNAL_SERVER_ERROR,
            Self::NotImplemented => riot_sys::COAP_CODE_NOT_IMPLEMENTED,
            Self::BadGateway => riot_sys::COAP_CODE_BAD_GATEWAY,
            Self::ServiceUnavailible => riot_sys::COAP_CODE_SERVICE_UNAVAILABLE,
            Self::GatewayTimeout => riot_sys::COAP_CODE_GATEWAY_TIMEOUT,
            Self::ProxyingNotSupported => riot_sys::COAP_CODE_PROXYING_NOT_SUPPORTED,
        }
    }
}

/// todo: enum useful? only one version is defined
/// todo: change getVersion()-signature if enum stays
pub enum CoapVersion {
    // Defined in RFC 7252
    // only version used in RIOT
    V1 = riot_sys::COAP_V1 as isize,
}

pub enum CoapContentFormat {
    Text,
    Link,
    Xml,
    Octet,
    Exi,
    Json,
    JsonPatchJson,
    MergePatchJson,
    Cbor,
    SenmlJson,
    SensmlJson,
    SenmlCbor,
    SensmlCbor,
    SenmlExi,
    SensmlExi,
    SenmlXml,
    SensmlXml,
    DnsMessage, /* Non-Standard! */
    None,
}

impl CoapContentFormat {
    pub fn from_c(input: libc::c_uint) -> Self {
        match input {
            riot_sys::COAP_FORMAT_TEXT => Self::Text,
            riot_sys::COAP_FORMAT_LINK => Self::Link,
            riot_sys::COAP_FORMAT_XML => Self::Xml,
            riot_sys::COAP_FORMAT_OCTET => Self::Octet,
            riot_sys::COAP_FORMAT_EXI => Self::Exi,
            riot_sys::COAP_FORMAT_JSON => Self::Json,
            riot_sys::COAP_FORMAT_JSON_PATCH_JSON => Self::JsonPatchJson,
            riot_sys::COAP_FORMAT_MERGE_PATCH_JSON => Self::MergePatchJson,
            riot_sys::COAP_FORMAT_CBOR => Self::Cbor,
            riot_sys::COAP_FORMAT_SENML_JSON => Self::SenmlJson,
            riot_sys::COAP_FORMAT_SENSML_JSON => Self::SensmlJson,
            riot_sys::COAP_FORMAT_SENML_CBOR => Self::SenmlCbor,
            riot_sys::COAP_FORMAT_SENSML_CBOR => Self::SensmlCbor,
            riot_sys::COAP_FORMAT_SENML_EXI => Self::SenmlExi,
            riot_sys::COAP_FORMAT_SENSML_EXI => Self::SensmlExi,
            riot_sys::COAP_FORMAT_SENML_XML => Self::SenmlXml,
            riot_sys::COAP_FORMAT_SENSML_XML => Self::SensmlXml,
            riot_sys::COAP_FORMAT_DNS_MESSAGE => Self::DnsMessage,

            _ => Self::None,
        }
    }

    pub fn to_c(self) -> libc::c_uint {
        match self {
            Self::Text => riot_sys::COAP_FORMAT_TEXT,
            Self::Link => riot_sys::COAP_FORMAT_LINK,
            Self::Xml => riot_sys::COAP_FORMAT_XML,
            Self::Octet => riot_sys::COAP_FORMAT_OCTET,
            Self::Exi => riot_sys::COAP_FORMAT_EXI,
            Self::Json => riot_sys::COAP_FORMAT_JSON,
            Self::JsonPatchJson => riot_sys::COAP_FORMAT_JSON_PATCH_JSON,
            Self::MergePatchJson => riot_sys::COAP_FORMAT_MERGE_PATCH_JSON,
            Self::Cbor => riot_sys::COAP_FORMAT_CBOR,
            Self::SenmlJson => riot_sys::COAP_FORMAT_SENML_JSON,
            Self::SensmlJson => riot_sys::COAP_FORMAT_SENSML_JSON,
            Self::SenmlCbor => riot_sys::COAP_FORMAT_SENML_CBOR,
            Self::SensmlCbor => riot_sys::COAP_FORMAT_SENSML_CBOR,
            Self::SenmlExi => riot_sys::COAP_FORMAT_SENML_EXI,
            Self::SensmlExi => riot_sys::COAP_FORMAT_SENSML_EXI,
            Self::SenmlXml => riot_sys::COAP_FORMAT_SENML_XML,
            Self::SensmlXml => riot_sys::COAP_FORMAT_SENSML_XML,
            Self::DnsMessage => riot_sys::COAP_FORMAT_DNS_MESSAGE, /* Non-Standard! */

            Self::None => riot_sys::COAP_FORMAT_NONE,
        }
    }
}

/// justified to have their own structs? {
pub struct CoapResource {
    resource: *mut riot_sys::coap_resource_t
}

pub struct CoapHandler {
    handler: *mut libc::c_void
}
/// }

pub struct CoapRequestContext {
    ctx: *mut riot_sys::coap_request_ctx_t
}

impl CoapRequestContext {
    // todo: find way to init safely
    pub unsafe fn new(remote: *mut riot_sys::sock_udp_ep_t) -> Self {
        let ctx: *mut riot_sys::coap_request_ctx_t;
        unsafe {
            ctx = MaybeUninit::uninit().assume_init_mut();
            riot_sys::coap_request_ctx_init(ctx, remote);
        }
        CoapRequestContext { ctx }
    }

    // todo: how to handle c_chars? lifetimes?
    pub fn get_path(&self) {
        let ret = unsafe { riot_sys::coap_request_ctx_get_path(self.ctx) };
    }

    // todo: giving back pointer ok?
    pub fn get_remote_udp(&self) -> *const riot_sys::sock_udp_ep_t {
        unsafe { riot_sys::coap_request_ctx_get_remote_udp(self.ctx) }
    }
}

pub struct CoapPacket {
    pkt: *mut riot_sys::coap_pkt_t,
    pkt_buf: *mut u8,
    buffer_len: usize
}

impl CoapPacket {
    ///@note: Wraps riot_sys::coap_pkt_init
    /// combine to new() function with coap_build_hdr
    ///     --> come up with buffer solution
    ///
    /// todo: add payload
    pub fn new(
        type_: CoapMessageType,
        token: Option<&[u8]>,
        code: CoapCode,
        id: u16,
        len: Option<usize>,
    ) -> Result<Self, CoapPacketError> {
        // todo: figure out lifetimes

        let buffer_len = match_len(len);

        // use of dynamic memory allocation allowed?
        let pkt_buf = unsafe { riot_sys::malloc(buffer_len as riot_sys::size_t) } as *mut u8;

        let token_len;
        let token_ = match token {
            Some(val) => {
                token_len = val.len();
                val
            }
            None => {
                token_len = 0;
                // todo: check rfc if this is a correct solution?
                &[0]
            }
        };

        let hdr_size = unsafe {
            riot_sys::coap_build_hdr(
                pkt_buf as *mut riot_sys::coap_hdr_t,
                type_.to_c(),
                token_.as_ptr() as *mut u8,
                token_len as riot_sys::size_t,
                code.to_c(),
                id,
            )
        };

        if hdr_size < 0 {
            return Err(CoapPacketError::HeaderBuildError);
        }

        let pkt: *mut riot_sys::coap_pkt_t;
        unsafe {
            pkt = MaybeUninit::uninit().assume_init_mut();
            riot_sys::coap_pkt_init(
                pkt,
                pkt_buf,
                buffer_len as riot_sys::size_t,
                hdr_size as riot_sys::size_t,
            );
        };

        Ok(CoapPacket {
            pkt,
            pkt_buf,
            buffer_len,
        })
    }

    pub fn reply_simple(
        &self,
        code: CoapCode,
        content_format: CoapContentFormat,
        len: Option<usize>,
        payload: &[u8],
    ) -> Result<&[u8], CoapPacketError> {
        let buffer_len = match len {
            Some(val) => val,
            None => COAP_DEFAULT_BUFFER_SIZE,
        };

        //let pkt_buffer: [u8; COAP_BUFFER_SIZE] = [0; COAP_BUFFER_SIZE];
        let pkt_buffer: *mut u8 =
            unsafe { riot_sys::malloc(buffer_len as riot_sys::size_t) as *mut u8 };

        if pkt_buffer.is_null() {
            return Err(CoapPacketError::MallocError);
        }

        let ret = unsafe {
            riot_sys::coap_reply_simple(
                self.pkt,
                code.to_c(),
                pkt_buffer,
                buffer_len as riot_sys::size_t,
                content_format.to_c(),
                payload.as_ptr() as *mut libc::c_void,
                payload.len() as riot_sys::size_t,
            )
        };

        if ret == (riot_sys::ENOSPC as i32) * -1 {
            Err(CoapPacketError::SmallBufferError)
        } else if ret < 0 {
            Err(CoapPacketError::ReplyError)
        } else {
            // todo: must be freed after! better solution?
            Ok(unsafe { core::slice::from_raw_parts(pkt_buffer, buffer_len) })
        }
    }

    pub fn build_reply(
        &self,
        code: CoapCode,
        resp_len: Option<usize>,
        payload_len: usize,
    ) -> Result<&[u8], CoapPacketError> {
        let len = match_len(resp_len);

        let resp_buf = unsafe { riot_sys::malloc(len as riot_sys::size_t) } as *mut u8;
        if resp_buf.is_null() {
            return Err(CoapPacketError::MallocError);
        }

        let ret = unsafe {
            riot_sys::coap_build_reply(
                self.pkt,
                code.to_c(),
                resp_buf,
                len as riot_sys::size_t,
                payload_len as riot_sys::size_t,
            )
        };

        if ret == (riot_sys::ENOSPC as i32) * -1 {
            unsafe {
                riot_sys::free(resp_buf as *mut core::ffi::c_void);
            }
            Err(CoapPacketError::SmallBufferError)
        } else if ret < 0 {
            unsafe {
                riot_sys::free(resp_buf as *mut core::ffi::c_void);
            }
            Err(CoapPacketError::ReplyError)
        } else {
            // todo: must be freed after! better solution?
            Ok(unsafe { core::slice::from_raw_parts_mut(resp_buf, len) })
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
    pub fn read_packet(buf: &[u8]) -> Result<CoapPacket, CoapPacketError> {
        unsafe {
            let pkt: *mut riot_sys::coap_pkt_t = MaybeUninit::uninit().assume_init_mut();

            if riot_sys::coap_parse(pkt, buf.as_ptr() as *mut u8, buf.len() as riot_sys::size_t) < 0
            {
                Err(CoapPacketError::ParseError)
            } else {
                Ok(CoapPacket {
                    pkt,
                    pkt_buf: buf.as_ptr() as *mut u8,
                    buffer_len: buf.len(),
                })
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
        CoapMessageType::from_c(unsafe {
            riot_sys::inline::coap_get_type(self.pkt as *const riot_sys::inline::coap_pkt_t)
        })
    }

    pub fn get_message_code_with_class(
        &self,
    ) -> Result<(CoapCodeClass, CoapCode), CoapPacketError> {
        let code = unsafe { (*(*self.pkt).hdr).code };

        let class = code << 5;
        let class_;

        match CoapCodeClass::from_c(class as u32) {
            Ok(val) => class_ = val,
            Err(e) => return Err(e),
        }

        match CoapCode::from_c(code as u32) {
            Ok(val) => Ok((class_, val)),
            Err(e) => Err(e),
        }
    }

    pub fn get_message_code_class(&self) -> Result<CoapCodeClass, CoapPacketError> {
        CoapCodeClass::from_c(unsafe {
            ((*(*self.pkt).hdr).code >> 5) as u32
        })
    }

    pub fn get_message_code(&self) -> Result<CoapCodeClass, CoapPacketError> {
        CoapCodeClass::from_c(unsafe {
            (*(*self.pkt).hdr).code as u32
        })
    }

    pub fn get_content_type(&self) -> CoapContentFormat {
        CoapContentFormat::from_c(unsafe { riot_sys::coap_get_content_type(self.pkt) })
    }

    pub fn get_accept(&self) -> CoapContentFormat {
        CoapContentFormat::from_c(unsafe { riot_sys::coap_get_accept(self.pkt) })
    }

    pub fn header_set_code(&self, code: CoapCode) {
        unsafe {
            (*(*self.pkt).hdr).code = code.to_c() as u8;
        }
    }

    pub fn header_set_type(&self, type_: CoapMessageType) {
        unsafe {
            riot_sys::inline::coap_hdr_set_type(
                (*self.pkt).hdr as *mut riot_sys::inline::coap_hdr_t,
                CoapMessageType::to_c(type_) as libc::c_uint,
            );
        }
    }

    pub fn put_option_empty(&self, last_opt_num: u16, new_op_num: u16) {
        unsafe {
            riot_sys::coap_put_option(
                self.pkt_buf,
                last_opt_num,
                new_op_num,
                riot_sys::inline::NULL as *const libc::c_void,
                0,
            );
        }
    }

    // todo: check boundaries
    pub fn put_option(&self, last_opt_num: u16, new_op_num: u16, data: &[u8]) {
        unsafe {
            riot_sys::coap_put_option(
                self.pkt_buf,
                last_opt_num,
                new_op_num,
                data.as_ptr() as *const libc::c_void,
                data.len() as riot_sys::size_t,
            );
        }
    }

    // todo: better way than to return pointer?
    pub fn get_payload_start(&self) -> *const u8 {
        unsafe {
            riot_sys::inline::coap_hdr_data_ptr(
                (*self.pkt).hdr as *const riot_sys::inline::coap_hdr_t,
            )
        }
    }

    pub fn handle_request(
        &self,
        resp_len: Option<usize>,
        ctx: *mut riot_sys::coap_request_ctx_t,
    ) -> Result<&[u8], CoapPacketError> {
        let len = match_len(resp_len);

        // malloc buffer here?
        let resp_buf: *mut u8 = unsafe { riot_sys::malloc(len as riot_sys::size_t) } as *mut u8;
        if resp_buf.is_null() {
            return Err(CoapPacketError::MallocError);
        }

        let ret =
            unsafe { riot_sys::coap_handle_req(self.pkt, resp_buf, len as riot_sys::size_t, ctx) };

        if ret == (riot_sys::EBADMSG as i32) * -1 {
            Err(CoapPacketError::NoRequestError)
        } else if ret < 0 {
            unsafe { riot_sys::free(resp_buf as *mut core::ffi::c_void) }
            Err(CoapPacketError::HandlingError)
        } else {
            // todo: must be freed after! better solution?
            Ok(unsafe { core::slice::from_raw_parts(resp_buf, len) })
        }
    }
}

fn match_len(opt: Option<usize>) -> usize {
    match opt {
        Some(val) => val,
        None => COAP_DEFAULT_BUFFER_SIZE,
    }
}

