//! https://api.riot-os.org/credman_8h_source.html
//! 
//! Authors: Henri Waller <henri@uni-bremen.de>, Lukas Terlau <terlau@uni-bremen.de> 


use riot_sys::libc::*;

use riot_sys::*;


#[derive(Debug)]
#[non_exhaustive]
pub enum CredmanStatus {
    CredmanOK,
    CredmanExist,
    CredmanNoSpace,
    CredmanNotFound,
    CredmanInvalid,
    CredmanTypeUnknown,
    CredmanError,
}

impl CredmanStatus {
    /// Converts the given `c_int` into the matching Enum representation
    fn from_c(n: c_int) -> Self {
        match n {
            0 => Self::CredmanOK,
            -1 => Self::CredmanExist,
            -2 => Self::CredmanNoSpace,
            -3 => Self::CredmanNotFound,
            -4 => Self::CredmanInvalid,
            -5 => Self::CredmanTypeUnknown,
            -6 => Self::CredmanError,
            // kann man sich die zahlen vielleicht auch aus dem C code nehmen?
        }
    }
}


#[derive(Debug)]
pub enum CredmanType {
    CredmanTypeEmpty,
    CredmanTypePSK,
    CredmanTypeECDSA,
}

impl CredmanType {
    /// Converts the given `c_int` into the matching Enum representation
    fn from_c(n: libc::c_uint) -> Self {
        match n {
            credman_type_t_CREDMAN_TYPE_EMPTYSelf => Self::CredmanTypeEmpty,
            credman_type_t_CREDMAN_TYPE_PSK => Self::CredmanTypePSK,
            credman_type_t_CREDMAN_TYPE_ECDSA =>  Self::CredmanTypeECDSA,
        }
    }
}

// int credman_add(const credman_credential_t *credential);
pub fn credman__add(cred: *const credman_credential_t) -> CredmanStatus{
    let res = unsafe {credman_add(cred)};
    CredmanStatus::from_c(res)
}

// int credman_get(credman_credential_t *credential, credman_tag_t tag, credman_type_t type);
pub fn credman__get(cred: *mut credman_credential_t, tag: credman_tag_t, typ: credman_type_t) -> Result<credman_credential_t,CredmanStatus>{
    let res = unsafe{credman_get(cred,tag,typ)};
    match CredmanStatus::from_c(res) {
        CredmanStatus::CredmanOK => Ok(unsafe {
            *cred   
        }),
        status => Err(status),
    }
}

// void credman_delete(credman_tag_t tag, credman_type_t type);
pub fn credman__delete(tag: credman_tag_t, typ: credman_type_t) {
    unsafe {
        credman_delete(tag,typ)
    };
}

// int credman_get_used_count(void);
pub fn credman__get_used_count() -> i32{
 unsafe{credman_get_used_count()}
}

// int credman_load_public_key(const void *buf, size_t buf_len, ecdsa_public_key_t *out);
pub fn load_public_key( buf : *const c_void , mut buf_len : usize, out : *mut ecdsa_public_key_t) -> Result<ecdsa_public_key_t,CredmanStatus>{
    unsafe {
        match CredmanStatus::from_c(credman_load_public_key(buf,buf_len,out)) {
            CredmanStatus::CredmanOK => Ok(*out),
            status => Err(status),
        }
    }
}

// int credman_load_private_key(const void *buf, size_t buf_len, credman_credential_t *cred);
pub fn load_private_key( buf : *const c_void , mut buf_len : usize, cred : *mut credman_credential_t) -> Result<credman_credential_t,CredmanStatus>{
    unsafe {
        match CredmanStatus::from_c(credman_load_private_key(buf,buf_len,cred)) {
            CredmanStatus::CredmanOK => Ok(*cred),
            status => Err(status),  
        }
    }
}

// int credman_load_private_ecc_key(const void *buf, size_t buf_len, credman_credential_t *cred);
pub fn load_private_ecc_key( buf : *const c_void , mut buf_len : usize, cred : *mut credman_credential_t) -> Result<credman_credential_t,CredmanStatus>{
    unsafe {
        match CredmanStatus::from_c(credman_load_private_ecc_key(buf,buf_len,cred)) {
            CredmanStatus::CredmanOK => Ok(*cred),
            status => Err(status),  
        }
    }
}
// #endif here

// #if here
// void credman_reset(void);
pub fn reset() {
    unsafe {
        credman_reset();
    }
}
// #endif here