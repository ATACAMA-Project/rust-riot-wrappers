//! https://api.riot-os.org/credman_8h_source.html
//! 
//! Authors: Henri Waller <henri@uni-bremen.de>, Lukas Terlau <terlau@uni-bremen.de> 




use riot_sys::libc::c_void;
use riot_sys::size_t;

pub struct CredmanCredential{
    cred: riot_sys::credman_credential_t,
}

pub struct CredmanBuffer{
    cred_buf: riot_sys::credman_buffer_t,
}

pub struct CredmanPskParams {
    psk: riot_sys::psk_params_t,
}

pub struct CredmanEcdsaPublicKey {
    ecdsa_public_key: riot_sys::ecdsa_public_key_t,
}

pub struct CredmanEcdsaParams {
    ecdsa: riot_sys::ecdsa_params_t ,
}

type CredmanTag = u16;


impl CredmanBuffer {
    pub fn new (s: *const c_void, len: usize) -> Self{
        CredmanBuffer {
            cred_buf: riot_sys::credman_buffer_t{
                s:s,
                len:len as size_t,
            }
        }
    }
}

impl CredmanPskParams {
    pub fn new (key: &CredmanBuffer, id: &CredmanBuffer, hint: &CredmanBuffer) -> Self {
        CredmanPskParams{
            psk: riot_sys::psk_params_t{
                key: key.cred_buf,
                id: id.cred_buf,
                hint: hint.cred_buf,
            }
        }
    }
}

impl CredmanEcdsaPublicKey {
    pub fn new ( x: *const c_void, y: *const c_void) -> Self {
        CredmanEcdsaPublicKey{
            ecdsa_public_key: riot_sys::ecdsa_public_key_t{
                x:x,
                y:y,
            }
        }
    }
}

impl CredmanEcdsaParams {
    pub fn new (private_key: *const c_void, public_key: &CredmanEcdsaPublicKey, client_key: &mut CredmanEcdsaPublicKey, client_keys_size: usize) -> Self {
        CredmanEcdsaParams{
            ecdsa: riot_sys::ecdsa_params_t {
                private_key: private_key,
                public_key: public_key.ecdsa_public_key,
                client_keys: &mut client_key.ecdsa_public_key as *mut riot_sys::ecdsa_public_key_t,
                client_keys_size: client_keys_size as size_t,
            }
        }
    }
}


impl CredmanCredential{
    pub fn new_psk ( tag: CredmanTag, psk: &CredmanPskParams) -> Self {
        CredmanCredential { 
            cred: riot_sys::credman_credential_t{
                type_:riot_sys::credman_type_t_CREDMAN_TYPE_PSK,
                tag: tag,
                params: riot_sys::credman_credential_t__bindgen_ty_1 {psk: psk.psk,},
            }
        }
    }
    pub fn new_ecdsa ( tag: CredmanTag, ecdsa: &CredmanEcdsaParams) -> Self {
        CredmanCredential { 
            cred: riot_sys::credman_credential_t{
                type_:riot_sys::credman_type_t_CREDMAN_TYPE_ECDSA,
                tag: tag,
                params: riot_sys::credman_credential_t__bindgen_ty_1 {ecdsa: ecdsa.ecdsa,},
            }
        }

    }
}





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
    CredmanStatusUnknown(i32),
}

impl CredmanStatus {
    /// Converts the given `i32` into the matching Enum representation
    fn from_value(n: i32) -> Self {
        match n {
            0 => Self::CredmanOK,
            -1 => Self::CredmanExist,
            -2 => Self::CredmanNoSpace,
            -3 => Self::CredmanNotFound,
            -4 => Self::CredmanInvalid,
            -5 => Self::CredmanTypeUnknown,
            -6 => Self::CredmanError,
            _ => Self::CredmanStatusUnknown(n),
        }
    }

    
}


#[derive(Debug)]
pub enum CredmanType {
    CredmanTypeEmpty,
    CredmanTypePSK,
    CredmanTypeECDSA,
    CredmanTypeUnknown(riot_sys::credman_type_t)
}

impl CredmanType {
    /// Converts the given `c_int` into the matching Enum representation
    fn from_value(n: riot_sys::credman_type_t) -> Self {
        match n {
            riot_sys::credman_type_t_CREDMAN_TYPE_EMPTY => Self::CredmanTypeEmpty,
            riot_sys::credman_type_t_CREDMAN_TYPE_PSK => Self::CredmanTypePSK,
            riot_sys::credman_type_t_CREDMAN_TYPE_ECDSA =>  Self::CredmanTypeECDSA,
            _ => Self::CredmanTypeUnknown(n),
        }
    }

    fn to_value(n: CredmanType) -> riot_sys::credman_type_t{
        match n {
            Self::CredmanTypeEmpty => riot_sys::credman_type_t_CREDMAN_TYPE_EMPTY,
            Self::CredmanTypePSK => riot_sys::credman_type_t_CREDMAN_TYPE_PSK,
            Self::CredmanTypeECDSA => riot_sys::credman_type_t_CREDMAN_TYPE_ECDSA,
            Self::CredmanTypeUnknown(i) => i, 
        }
    }
}

// int credman_add(const credman_credential_t *credential);
pub fn credman_add(cred: &CredmanCredential) -> CredmanStatus{
    let res = unsafe {riot_sys::credman_add(&cred.cred as *const riot_sys::credman_credential_t)};
    CredmanStatus::from_value(res)
}

// int credman_get(credman_credential_t *credential, credman_tag_t tag, credman_type_t type);
pub fn credman_get( tag: CredmanTag, typ: CredmanType) -> Result<CredmanCredential,CredmanStatus>{
    let mut cred : riot_sys::credman_credential_t = Default::default();
    let res = unsafe{riot_sys::credman_get(&mut cred as *mut riot_sys::credman_credential_t,tag,CredmanType::to_value(typ))};
    match CredmanStatus::from_value(res) {
        CredmanStatus::CredmanOK => Ok(CredmanCredential { cred: cred }),
        status => Err(status),
    }
}

// void credman_delete(credman_tag_t tag, credman_type_t type);
pub fn credman_delete(tag: CredmanTag, typ: CredmanType) {
    unsafe {
        riot_sys::credman_delete(tag,CredmanType::to_value(typ))
    };
}

// int credman_get_used_count(void);
pub fn credman_get_used_count() -> u32{
 unsafe{riot_sys::credman_get_used_count().try_into().unwrap()}
}


/* 
// int credman_load_public_key(const void *buf, size_t buf_len, ecdsa_public_key_t *out);
#[cfg(riot_module_credman_load)]
pub fn credman_load_public_key( buf : *const c_void , mut buf_len : u32, out : *mut riot_sys::ecdsa_public_key_t) -> Result<riot_sys::ecdsa_public_key_t,CredmanStatus>{
    unsafe {
        match CredmanStatus::from_value(riot_sys::credman_load_public_key(buf,buf_len,out)) {
            CredmanStatus::CredmanOK => Ok(*out),
            status => Err(status),
        }
    }
}

// int credman_load_private_key(const void *buf, size_t buf_len, credman_credential_t *cred);
#[cfg(riot_module_credman_load)]
pub fn credman_load_private_key( buf : *const c_void , mut buf_len : u32) -> Result<CredmanCredential,CredmanStatus>{
    let mut cred : riot_sys::credman_credential_t = Default::default();
    unsafe {
        match CredmanStatus::from_value(riot_sys::credman_load_private_key(buf,buf_len,&mut cred as *mut riot_sys::credman_credential_t)) {
            CredmanStatus::CredmanOK => Ok(CredmanCredential { cred: cred }),
            status => Err(status),  
        }
    }
}

// int credman_load_private_ecc_key(const void *buf, size_t buf_len, credman_credential_t *cred);
#[cfg(riot_module_credman_load)]
pub fn credman_load_private_ecc_key( buf : *const c_void , mut buf_len : u32) -> Result<CredmanCredential,CredmanStatus>{
    let mut cred : riot_sys::credman_credential_t = Default::default();
    unsafe {
        match CredmanStatus::from_value(riot_sys::credman_load_private_ecc_key(buf,buf_len,&mut cred as *mut riot_sys::credman_credential_t)) {
            CredmanStatus::CredmanOK => Ok(CredmanCredential { cred: cred }),
            status => Err(status),  
        }
    }
}
// #endif here

// #if here
// void credman_reset(void);
/*pub fn credman_reset() {
    unsafe {
        riot_sys::credman_reset();
    }
}*/
// #endif here

*/