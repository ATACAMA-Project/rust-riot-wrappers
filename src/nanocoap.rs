//! Wrapper containing structs and functions from nanocoap.h
//!
use core::mem::MaybeUninit;
use {riot_sys, riot_sys::libc};
use crate::println;
use core::ffi::CStr;
use core::ptr;
use riot_sys::{COAP_CODE_205, COAP_FORMAT_TEXT, coap_pkt_t, coap_reply_simple, coap_request_ctx_t, coap_resource_t, RIOT_BOARD, size_t, ssize_t};



pub struct CoapResource {
    path: &'static CStr,
    methods: CoapMethod,
    handler: unsafe extern "C" fn(
        pkt: *mut coap_pkt_t,
        buf: *mut u8,
        len: size_t,
        context: *mut coap_request_ctx_t,
    ) -> ssize_t,
}

impl CoapResource{

    pub const fn new(path: &'static CStr,
                     methods: CoapMethod,
                     handler: unsafe extern "C" fn (pkt: *mut coap_pkt_t, buf: *mut u8, len: size_t, context: *mut coap_request_ctx_t) -> ssize_t) -> Self {
        Self {
            path,
            methods,
            handler,
        }
    }

    pub const fn to_c(self) -> coap_resource_t{
        coap_resource_t{
            path: self.path.as_ptr() as _,
            methods: self.methods.to_c() as u16,
            handler: Some(self.handler),
            context: ptr::null_mut(),
        }
    }
}



/// Enum containing the possible methods for a CoapPacket
///
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
    /// Translates the constants from the C-Bindings to CoapMethod enum entries
    ///
    /// # Arguments
    ///
    /// * `input` - The to be translated constant from C-Bindings
    ///
    /// # Return
    ///
    /// Returns a Result containing either the matching enum entry from CoapMethod or a
    /// MethodMatchError if the constant couldn't be matched
    ///
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

    /// Translates CoapMethod enum entries to constants from the C-Bindings
    ///
    /// # Arguments
    ///
    /// * `self` - The to be translated enum entry
    ///
    /// # Return
    ///
    /// Returns the matching constant from the C-Bindings
    ///
    pub const fn to_c(self) -> u32 {
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

/// Enum containing the possible message types for CoapPackets
///
pub enum CoapMessageType {
    Con,
    Non,
    Ack,
    Rst,
}

impl CoapMessageType {
    /// Translates the constants from the C-Bindings to CoapMessageType enum entries
    ///
    /// # Arguments
    ///
    /// * `input` - The to be translated constant from C-Bindings
    ///
    /// # Return
    ///
    /// Returns a Result containing either the matching enum entry from CoapMessageType or a
    /// TypeMatchError if the constant couldn't be matched
    ///
    pub fn from_c(input: u32) -> Result<Self, CoapPacketError> {
        match input {
            riot_sys::COAP_TYPE_CON => Ok(Self::Con),
            riot_sys::COAP_TYPE_NON => Ok(Self::Non),
            riot_sys::COAP_TYPE_ACK => Ok(Self::Ack),
            riot_sys::COAP_TYPE_RST => Ok(Self::Rst),
            _ => Err(CoapPacketError::TypeMatchError),
        }
    }

    /// Translates CoapMessageType enum entries to constants from the C-Bindings
    ///
    /// # Arguments
    ///
    /// * `self` - The to be translated enum entry
    ///
    /// # Return
    ///
    /// Returns the matching constant from the C-Bindings
    ///
    pub fn to_c(self) -> u32 {
        match self {
            Self::Con => riot_sys::COAP_TYPE_CON,
            Self::Non => riot_sys::COAP_TYPE_NON,
            Self::Ack => riot_sys::COAP_TYPE_ACK,
            Self::Rst => riot_sys::COAP_TYPE_RST,
        }
    }
}

/// Enum containing the possible Code Classes for CoapPackets
///
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
    /// Translates the constants from the C-Bindings to CoapCodeClass enum entries
    ///
    /// # Arguments
    ///
    /// * `input` - The to be translated constant from C-Bindings
    ///
    /// # Return
    ///
    /// Returns a Result containing either the matching enum entry from CoapMessageType or a
    /// CodeMatchError if the constant couldn't be matched
    ///
    pub fn from_c(input: u32) -> Result<Self, CoapPacketError> {
        match input {
            riot_sys::COAP_CLASS_REQ => Ok(Self::Request),
            riot_sys::COAP_CLASS_SUCCESS => Ok(Self::Success),
            riot_sys::COAP_CLASS_CLIENT_FAILURE => Ok(Self::ClientError),
            riot_sys::COAP_CLASS_SERVER_FAILURE => Ok(Self::ServerError),
            _ => Err(CoapPacketError::CodeMatchError),
        }
    }

    /// Translates CoapCodeClass enum entries to constants from the C-Bindings
    ///
    /// # Arguments
    ///
    /// * `self` - The to be translated enum entry
    ///
    /// # Return
    ///
    /// Returns the matching constant from the C-Bindings
    ///
    pub fn to_c(self) -> u32 {
        match self {
            Self::Request => riot_sys::COAP_CLASS_REQ,
            Self::Success => riot_sys::COAP_CLASS_SUCCESS,
            Self::ClientError => riot_sys::COAP_CLASS_CLIENT_FAILURE,
            Self::ServerError => riot_sys::COAP_CLASS_SERVER_FAILURE,
        }
    }
}

/// Enum containing the possible Message Codes for CoapPackets
///
pub enum CoapCode {
    // Requests (0.x)
    // 0.00
    EmptyRequest,

    Get,

    Put,

    Post,

    Delete,

    Fetch,

    Patch,

    IPatch,

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
    /// Translates the constants from the C-Bindings to CoapCode enum entries
    ///
    /// # Arguments
    ///
    /// * `input` - The to be translated constant from C-Bindings
    ///
    /// # Return
    ///
    /// Returns a Result containing either the matching enum entry from CoapCode or a
    /// CodeMatchError if the constant couldn't be matched
    ///
    pub fn from_c(input: u32) -> Result<CoapCode, CoapPacketError> {
        match input {
            riot_sys::COAP_CODE_EMPTY => Ok(Self::EmptyRequest),
            riot_sys::COAP_METHOD_GET => Ok(Self::Get),
            riot_sys::COAP_METHOD_PUT => Ok(Self::Put),
            riot_sys::COAP_METHOD_POST => Ok(Self::Post),
            riot_sys::COAP_METHOD_DELETE => Ok(Self::Delete),
            riot_sys::COAP_METHOD_FETCH => Ok(Self::Fetch),
            riot_sys::COAP_METHOD_PATCH => Ok(Self::Patch),
            riot_sys::COAP_METHOD_IPATCH => Ok(Self::IPatch),
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

    /// Translates CoapCode enum entries to constants from the C-Bindings
    ///
    /// # Arguments
    ///
    /// * `self` - The to be translated enum entry
    ///
    /// # Return
    ///
    /// Returns the matching constant from the C-Bindings
    ///
    pub fn to_c(self) -> u32 {
        match self {
            Self::EmptyRequest => riot_sys::COAP_CODE_EMPTY,
            Self::Get => riot_sys::COAP_METHOD_GET,
            Self::Put => riot_sys::COAP_METHOD_PUT,
            Self::Post => riot_sys::COAP_METHOD_POST,
            Self::Delete => riot_sys::COAP_METHOD_DELETE,
            Self::Fetch => riot_sys::COAP_METHOD_FETCH,
            Self::Patch => riot_sys::COAP_METHOD_PATCH,
            Self::IPatch => riot_sys::COAP_METHOD_IPATCH,
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

/// Enum containing the possible Version identifiers for CoapPackets
///
pub enum CoapVersion {
    // Defined in RFC 7252
    // only version used in RIOT
    V1 = riot_sys::COAP_V1 as isize,
}

impl CoapVersion {
    /// Translates the constants from the C-Bindings to CoapVersion enum entries
    ///
    /// # Arguments
    ///
    /// * `input` - The to be translated constant from C-Bindings
    ///
    /// # Return
    ///
    /// Returns a Result containing either the matching enum entry from CoapVersion or a
    /// VersionUnknownError if the constant couldn't be matched
    ///
    pub fn from_c(input: u32) -> Result<Self, CoapPacketError> {
        match input {
            riot_sys::COAP_V1 => Ok(Self::V1),
            _ => Err(CoapPacketError::VersionUnknownError),
        }
    }

    /// Translates CoapVersion enum entries to constants from the C-Bindings
    ///
    /// # Arguments
    ///
    /// * `self` - The to be translated enum entry
    ///
    /// # Return
    ///
    /// Returns the matching constant from the C-Bindings
    ///
    pub fn to_c(self) -> u32 {
        match self {
            Self::V1 => riot_sys::COAP_V1,
        }
    }
}

/// Enum containing the possible Content Format identifier for CoapPackets
///
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
    /// Translates the constants from the C-Bindings to CoapContentFormat enum entries
    ///
    /// # Arguments
    ///
    /// * `input` - The to be translated constant from C-Bindings
    ///
    /// # Return
    ///
    /// Returns either the matching enum entry from CoapContentFormat or CoapContentFormat::None if
    /// the constant couldn't be matched
    ///
    pub fn from_c(input: u32) -> Self {
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

    /// Translates CoapContentFormat enum entries to constants from the C-Bindings
    ///
    /// # Arguments
    ///
    /// * `self` - The to be translated enum entry
    ///
    /// # Return
    ///
    /// Returns the matching constant from the C-Bindings
    ///
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

// todo: figure out resource handling in rust

/// Struct used to store the CoAP resource request handler context
///
pub struct CoapRequestContext {
    ctx: riot_sys::coap_request_ctx_t,
}

impl CoapRequestContext {
    // todo: find way to init safely
    /// Creates and initializes a Coap Request Context and stores it in a CoapRequestContext
    ///
    /// # Arguments
    ///
    /// * `remote` - The pointer to endpoint of the request
    ///
    /// # Returns
    ///
    /// Returns a CoapRequestContext struct that contains the initialized request context
    ///
    pub fn new(remote: &mut riot_sys::sock_udp_ep_t) -> Self {
        unsafe {
            let mut ctx = MaybeUninit::<riot_sys::coap_request_ctx_t>::uninit();
            riot_sys::coap_request_ctx_init(
                ctx.assume_init_mut() as *mut riot_sys::coap_request_ctx_t,
                remote,
            );
            Self {
                ctx: ctx.assume_init(),
            }
        }
    }

    /*
    pub fn get_path(&self) -> String {
        // todo: implement
        // todo: how to handle c_chars? lifetimes?
        let ret = unsafe { riot_sys::coap_request_ctx_get_path(self.ctx) };
    }

     */

    // todo: alter to not give back pointer
    pub fn get_remote_udp(&self) -> *const riot_sys::sock_udp_ep_t {
        unsafe {
            riot_sys::coap_request_ctx_get_remote_udp(
                &self.ctx as *const riot_sys::coap_request_ctx_t,
            )
        }
    }

    // todo: alter to not give back pointer
    pub unsafe fn get_context(&self) -> *mut libc::c_void {
        unsafe { (*self.ctx.resource).context }
    }
}

/// struct containing the CoAP PDU parsing context structure and the packet buffer
///
// todo: Lifetime specifiers needed?
// todo: Dropping?
pub struct CoapPacket {
    pkt: *mut riot_sys::coap_pkt_t,
    pub pkt_buffer: *mut u8,
    pub pkt_len: usize,
}

/// Enum containing Possible errors of CoapPacket
///
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
    AllocationError,
    HandlingError,
    NoRequestError,
    VersionUnknownError,
    TokenSizeError,
    PacketTooBigError,
    PayloadTooBigError,
    OptionError,
}

impl CoapPacket {
    /// Creates a CoapPacket, and fills the parsing structure and packet buffer according to the
    /// arguments
    ///
    /// # Arguments
    ///
    /// * `type_` - The type of the packet (e.g. Con, Non, ...)
    ///
    /// * `token_opt` - An Option, which contains either None if the token length is supposed to be 0
    ///             or Some with the token value as &[u8]
    ///
    /// * `code` - The message code for the packet
    ///
    /// * `id` - The id for the packet
    ///
    /// * `len` - An Option, which contains either None if the default package size should be
    ///           chosen or Some with an usize value which contains the chosen size for the package
    ///           buffer
    ///
    /// # Returns
    ///
    /// Returns a Result with either an Ok with a CoapPacket with an allocated package buffer with
    /// initialized header or an Err with a CoapPacketError
    ///
    /// # Errors
    ///
    /// * `AllocationError` - If an error occurs when allocating the package buffer
    ///
    /// * `HeaderBuildError` - If an error occurs while building the header
    ///
    // todo: zeroing of the out-buf necessary?
    // todo: method2flag needed?
    // todo: add options
    pub fn new(
        buffer: &mut [u8],
        type_: CoapMessageType,
        token_opt: Option<&[u8]>,
        code_opt: Option<CoapCode>,
        id: u16,
        payload_opt: Option<&[u8]>,
    ) -> Result<Self, CoapPacketError> {
        let token_len;
        let token = match token_opt {
            Some(val) => {
                if val.len() <= 8 {
                    token_len = val.len();
                } else {
                    return Err(CoapPacketError::TokenSizeError);
                }
                val
            }
            None => {
                token_len = 0;
                &[0]
            }
        };

        // return if buffer can't hold the header
        if buffer.len() < 4 + token_len {
            return Err(CoapPacketError::SmallBufferError);
        }

        let code = match code_opt {
            Some(val) => val,
            None => CoapCode::EmptyRequest,
        };

        let hdr_size = unsafe {
            riot_sys::coap_build_hdr(
                buffer.as_mut_ptr() as *mut riot_sys::coap_hdr_t,
                type_.to_c(),
                token.as_ptr() as *mut u8,
                token_len as riot_sys::size_t,
                code.to_c(),
                id,
            )
        };

        let mut pkt = MaybeUninit::<riot_sys::coap_pkt_t>::uninit();

        unsafe {
            riot_sys::coap_pkt_init(
                pkt.as_mut_ptr() as *mut riot_sys::coap_pkt_t,
                buffer.as_mut_ptr() as *mut u8,
                buffer.len() as riot_sys::size_t,
                hdr_size as riot_sys::size_t,
            );
        }

        // todo: add option input
        let optnum: u16 = 0;

        // set payload marker if payload will be written
        if payload_opt.is_some() {
            let ret = unsafe {
                riot_sys::coap_opt_finish(
                    pkt.assume_init_mut() as *mut riot_sys::coap_pkt_t,
                    optnum,
                )
            };
            if ret < 0 {
                return Err(CoapPacketError::OptionError);
            }
        }

        let payload_len = match payload_opt {
            Some(val) => val.len(),
            None => 0,
        };

        if payload_len > 0 {
            let ret = unsafe {
                riot_sys::coap_payload_put_bytes(
                    pkt.assume_init_mut(),
                    payload_opt.unwrap().as_ptr() as *const core::ffi::c_void,
                    payload_len as riot_sys::size_t,
                )
            };
            if ret < 0 {
                return Err(CoapPacketError::PayloadWriteError);
            }
        }

        Ok(Self {
            pkt: unsafe { pkt.as_mut_ptr() },
            pkt_buffer: buffer.as_mut_ptr(),
            pkt_len: buffer.len(),
        })
    }

    pub fn from(pkt: *mut riot_sys::coap_pkt_t, pkt_buffer: *mut u8, pkt_len: usize) -> Self{
        CoapPacket{
            pkt,
            pkt_buffer,
            pkt_len,
        }
    }

    /// Builds a reply to a received CoapPacket with payload
    ///
    /// # Arguments
    ///
    /// * `self` - The received CoapPacket that we generate a reply to
    ///
    /// * `code` - The message code for the reply
    ///
    /// * `content_format_opt` - The Option for the content_format of the payload. None if the payload
    ///                      isn't in an (in the enum) specified format, Some if an enum entry
    ///                      matches the format of the payload
    ///
    /// * `len` - The length of the to be allocated packet buffer
    ///
    /// * `payload` - The payload as byte-slice
    ///
    /// # Returns
    ///
    /// Returns a Result containing either an Ok with the resulting packet buffer including the payload as
    /// byte-slice on success or an Err with the corresponding CoapPacketError on failure
    ///
    /// # Safety
    ///
    /// The resulting byte-slice currently has to be freed after sending the resulting package
    ///
    /// # Errors
    ///
    /// * `AllocationError` - If an error occurs while allocating the package buffer
    ///
    /// * `SmallBufferError` - If the allocated buffer is too small to hold the resulting package
    ///
    /// * `ReplyError` - If other errors occurred while building the reply
    ///
    pub fn reply_simple(
        &mut self,
        code: CoapCode,
        content_format_opt: Option<CoapContentFormat>,
        payload: &[u8],
    ) -> Result<usize, CoapPacketError> {
        let content_format = match content_format_opt {
            Some(val) => val,
            None => CoapContentFormat::None,
        };

        println!("{:?}", self.pkt_len as size_t);

        let ret = unsafe {
            riot_sys::coap_reply_simple(
                self.pkt,
                code.to_c(),
                self.pkt_buffer,
                self.pkt_len as _,
                content_format.to_c(),
                payload.as_ptr() as *const _,
                payload.len() as _,
            )
        };

        if ret == -(riot_sys::ENOSPC as i32) {
            Err(CoapPacketError::SmallBufferError)
        } else if ret < 0 {
            Err(CoapPacketError::ReplyError)
        } else {
            Ok(self.pkt_len)
        }
    }

    /// Builds a reply to a received CoapPacket with space for a payload
    ///
    /// # Arguments
    ///
    /// * `self` - The received CoapPacket that we generate a reply to
    ///
    /// * `code` - The message code for the reply
    ///
    /// * `resp_len` - Option containing None if the resulting buffer should have the default
    ///                buffer size or Some with a usize value with the length of the to be allocated
    ///                packet buffer
    ///
    /// * `payload_len` - Option containing None if the resulting packet shall contain a payload or
    ///                   Some with a usize value with the supposed payload length
    ///
    /// # Returns
    ///
    /// Returns a Result containing either an Ok with the resulting packet buffer with space for the
    /// payload as byte-slice on success or an Err with the corresponding CoapPacketError on failure
    ///
    /// # Safety
    ///
    /// The resulting byte-slice currently has to be freed after sending the resulting package
    ///
    /// # Errors
    ///
    /// * `AllocationError` - If an error occurs while allocating the package buffer
    ///
    /// * `SmallBufferError` - If the allocated buffer is too small to hold the resulting package
    ///
    /// * `ReplyError` - If other errors occurred while building the reply
    ///
    pub fn reply_no_payload(
        &mut self,
        resp_buffer: &mut [u8],
        code: CoapCode,
    ) -> Result<(), CoapPacketError> {
        let ret = unsafe {
            riot_sys::coap_build_reply(
                self.pkt as *mut riot_sys::coap_pkt_t,
                code.to_c(),
                resp_buffer.as_ptr() as *mut u8,
                resp_buffer.len() as riot_sys::size_t,
                0 as riot_sys::size_t,
            )
        };

        if ret == -(riot_sys::ENOSPC as i32) {
            Err(CoapPacketError::SmallBufferError)
        } else if ret < 0 {
            Err(CoapPacketError::ReplyError)
        } else {
            Ok(())
        }
    }

    /// Parse a CoAP PDU into a CoapPacket struct
    ///
    /// # Arguments
    ///
    /// * `buf` - The byte-slice containing a received CoAP-Message
    ///
    /// # Return
    ///
    /// Returns a Result with either Ok with the parsed CoapPacket or an Err with a ParseError
    /// otherwise
    ///
    pub fn read_packet(buf: &mut [u8]) -> Result<Self, CoapPacketError> {
        let mut pkt = unsafe { MaybeUninit::<riot_sys::coap_pkt_t>::uninit() };

        if unsafe {
            riot_sys::coap_parse(
                pkt.as_mut_ptr() as *mut riot_sys::coap_pkt_t,
                buf.as_ptr() as *mut u8,
                buf.len() as riot_sys::size_t,
            )
        } < 0
        {
            Err(CoapPacketError::ParseError)
        } else {
            Ok(Self {
                pkt: unsafe { pkt.as_mut_ptr() },
                pkt_buffer: buf.as_mut_ptr(),
                pkt_len: buf.len(),
            })
        }
    }

    pub fn get_version(&self) -> Result<CoapVersion, CoapPacketError> {
        CoapVersion::from_c(unsafe { riot_sys::inline::coap_get_ver(crate::inline_cast(self.pkt)) })
        //CoapVersion::from_c( unsafe { riot_sys::inline::coap_get_ver(&self.pkt as *const riot_sys::inline::coap_pkt_t) })
    }

    pub fn get_token_len(&self) -> usize {
        unsafe { riot_sys::inline::coap_get_token_len(crate::inline_cast(self.pkt)) as usize }
    }

    pub fn get_token(&self) -> Result<Option<&[u8]>, CoapPacketError> {
        let token_len =
            unsafe { riot_sys::inline::coap_get_token_len(crate::inline_cast(self.pkt)) };
        let token = unsafe { riot_sys::inline::coap_get_token(crate::inline_cast(self.pkt)) };
        if token_len > 8 {
            Err(CoapPacketError::TokenSizeError)
        } else {
            match token_len {
                0 => Ok(None),
                _ => Ok(Some(unsafe {
                    core::slice::from_raw_parts(token as *const u8, token_len as usize)
                })),
            }
        }
    }

    pub fn get_id(&self) -> u32 {
        unsafe { riot_sys::inline::coap_get_id(crate::inline_cast(self.pkt)) }
    }

    /*
    pub fn get_type(&self) -> Result<CoapMessageType, CoapPacketError> {
        CoapMessageType::from_c(unsafe { ((*self.pkt.hdr).ver_t_tkl & 0x30) >> 4 } as u32)
    }

     */

    pub fn get_message_code_with_class(
        &self,
    ) -> Result<(CoapCodeClass, CoapCode), CoapPacketError> {
        let detail = unsafe {
            // todo: right method?
            riot_sys::inline::coap_get_code_detail(crate::inline_cast(self.pkt))
        };
        let class = unsafe { riot_sys::inline::coap_get_code_class(crate::inline_cast(self.pkt)) };

        match CoapCodeClass::from_c(detail) {
            Ok(val) => match CoapCode::from_c(class) {
                Ok(val_) => Ok((val, val_)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_message_code_class(&self) -> Result<CoapCodeClass, CoapPacketError> {
        CoapCodeClass::from_c(unsafe {
            riot_sys::inline::coap_get_code_class(crate::inline_cast(self.pkt))
        })
    }

    pub fn get_message_code(&self) -> Result<CoapCode, CoapPacketError> {
        CoapCode::from_c(unsafe {
            riot_sys::inline::coap_get_code_detail(crate::inline_cast(self.pkt))
        })
    }

    pub fn get_content_type(&mut self) -> CoapContentFormat {
        CoapContentFormat::from_c(unsafe { riot_sys::coap_get_content_type(self.pkt) })
    }

    pub fn get_accept(&mut self) -> CoapContentFormat {
        CoapContentFormat::from_c(unsafe { riot_sys::coap_get_accept(self.pkt) })
    }

    // todo: implement
    // todo: find way to appropriately give back C-String
    /*
    pub fn get_uri_path(&mut self) {
        unsafe {
            riot_sys::inline::coap_get_uri_path(self as *mut riot_sys::inline::coap_pkt_t, );
        }
    }

     */

    pub fn header_set_code(&self, code: CoapCode) {
        unsafe {
            riot_sys::inline::coap_hdr_set_code(
                crate::inline_cast_mut((*self.pkt).hdr),
                code.to_c() as u8,
            );
        }
    }

    pub fn header_set_type(&self, type_: CoapMessageType) {
        unsafe {
            riot_sys::inline::coap_hdr_set_type(
                crate::inline_cast_mut((*self.pkt).hdr),
                CoapMessageType::to_c(type_) as libc::c_uint,
            );
        }
    }

    // todo: options should be tracked in a seperate struct
    pub fn put_option(&self, last_opt_num: u16, new_op_num: u16, data: &[u8]) {
        unsafe {
            riot_sys::coap_put_option(
                self.pkt_buffer,
                last_opt_num,
                new_op_num,
                data.as_ptr() as *const libc::c_void,
                data.len() as riot_sys::size_t,
            );
        }
    }

    pub fn tree_handler(
        &mut self,
        ctx: &mut CoapRequestContext,
        resp_buffer: &mut [u8],
        resource: &riot_sys::coap_resource_t,
        resource_len: usize,
    ) -> Result<usize, CoapPacketError> {
        unsafe {
            let ret = riot_sys::coap_tree_handler(
                self.pkt,
                resp_buffer.as_ptr() as *mut u8,
                resp_buffer.len() as riot_sys::size_t,
                &mut ctx.ctx,
                resource,
                resource_len as riot_sys::size_t,
            );
            if ret == -(riot_sys::ENOSPC as riot_sys::ssize_t) {
                Err(CoapPacketError::SmallBufferError)
            } else if ret < 0 {
                Err(CoapPacketError::TreeHandleError)
            } else {
                Ok(ret as usize)
            }
        }
    }

    pub fn handle_request(
        &mut self,
        ctx: &mut CoapRequestContext,
        resp_buffer: &mut [u8],
    ) -> Result<(), CoapPacketError> {
        let ret = unsafe {
            riot_sys::coap_handle_req(
                self.pkt,
                resp_buffer.as_ptr() as *mut u8,
                resp_buffer.len() as riot_sys::size_t,
                &mut ctx.ctx as *mut riot_sys::coap_request_ctx_t,
            )
        };

        if ret == -(riot_sys::EBADMSG as i32) {
            Err(CoapPacketError::NoRequestError)
        } else if ret < 0 {
            Err(CoapPacketError::HandlingError)
        } else {
            Ok(())
        }
    }
}
