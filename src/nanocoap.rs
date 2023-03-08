//! Wrapper containing structs and functions from nanocoap.h
//!
use crate::helpers::SliceToCStr;
use core::ffi::{c_char, c_int, c_void};



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

pub struct CoapResource {
    resource: *mut riot_sys::coap_resource_t
}

pub struct CoapHandler {
    handler: *mut libc::c_void
}

pub struct CoapRequestContext {
    ctx: *mut riot_sys::coap_request_ctx_t
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