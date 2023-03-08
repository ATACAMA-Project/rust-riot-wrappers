//! Wrapper containing structs and functions from nanocoap.h
//!
use crate::helpers::SliceToCStr;
use core::ffi::{c_char, c_int, c_void};



//! Relevant structs from nanocoap.h

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
    fn new(path: &str, flags: CoapMethodFlagT, handler: CoapHandler, ctx: CoapRequestContext) -> CoapResource {
        let _path = path.as_bytes().to_cstr().unwrap().as_ptr();
        let _flags: riot_sys::coap_method_flags_t;
        let _handler: riot_sys::coap_handler_t;
        let _context: *mut c_void;
        
        return CoapResource {
            resource: riot_sys::coap_resource_t {
                path: path.as_bytes().to_cstr().unwrap().as_ptr(),
                methods: flags as riot_sys::coap_method_flags_t,
                handler: handler.handler,
                context: ctx.ctx
            }
        };
    }
}