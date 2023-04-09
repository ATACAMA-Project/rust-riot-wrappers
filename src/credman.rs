//! https://api.riot-os.org/credman_8h_source.html
//!
//! Authors: Henri Waller <henri@uni-bremen.de>, Lukas Terlau <terlau@uni-bremen.de>

use core::ptr;
use riot_sys::libc::c_void;
use riot_sys::size_t;

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
    CredmanStatusUnknown,
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
            _ => Self::CredmanStatusUnknown,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CredmanType {
    CredmanTypeEmpty,
    CredmanTypePSK,
    CredmanTypeECDSA,
}

impl CredmanType {
    fn to_c(self) -> riot_sys::credman_type_t {
        match self {
            Self::CredmanTypeEmpty => riot_sys::credman_type_t_CREDMAN_TYPE_EMPTY,
            Self::CredmanTypePSK => riot_sys::credman_type_t_CREDMAN_TYPE_PSK,
            Self::CredmanTypeECDSA => riot_sys::credman_type_t_CREDMAN_TYPE_ECDSA,
        }
    }
}

pub struct CredentialRef {
    pub credential: riot_sys::credman_credential_t,
}

type CredmanTag = u16;

pub enum Params<'a> {
    Psk(PskParams<'a>),
    Ecdsa(EcdsaParams<'a>),
}

pub struct PskParams<'a> {
    pub key: &'a [u8],
    pub id: &'a [u8],
    pub hint: &'a [u8],
}

pub struct EcdsaPublicKey<'a> {
    pub x: &'a [u8],
    pub y: &'a [u8],
}

pub struct EcdsaParams<'a> {
    pub private_key: &'a [u8],
    pub public_key: EcdsaPublicKey<'a>,
    client_keys: &'a [riot_sys::ecdsa_public_key_t],
}

impl<'a> EcdsaParams<'a> {
    pub fn new<const EC_CLIENT_KEYS_NUM: usize>(
        private_key: &'a [u8],
        public_key: EcdsaPublicKey<'a>,
        client_keys: &'a EcdsaClientKeys<EC_CLIENT_KEYS_NUM>,
    ) -> Self {
        EcdsaParams {
            private_key,
            public_key,
            client_keys: &client_keys.client_keys_c,
        }
    }
}

pub struct EcdsaClientKeys<const EC_CLIENT_KEYS_NUM: usize> {
    client_keys_c: [riot_sys::ecdsa_public_key_t; EC_CLIENT_KEYS_NUM],
}

impl<const EC_CLIENT_KEYS_NUM: usize> EcdsaClientKeys<EC_CLIENT_KEYS_NUM> {
    fn new(client_keys: &[EcdsaPublicKey]) -> Self {
        let mut keys = EcdsaClientKeys {
            client_keys_c: [riot_sys::ecdsa_public_key_t {
                x: ptr::null(),
                y: ptr::null(),
            }; EC_CLIENT_KEYS_NUM],
        };
        for (i, key) in client_keys.iter().enumerate() {
            keys.client_keys_c[i] = riot_sys::ecdsa_public_key_t {
                x: key.x.as_ptr() as *const c_void,
                y: key.y.as_ptr() as *const c_void,
            };
        }
        keys
    }
}

pub struct Credential<'a> {
    credential: riot_sys::credman_credential_t,
    params: Params<'a>,
}

impl<'a> Credential<'a> {
    pub fn new_psk(tag: CredmanTag, psk: PskParams<'a>) -> Self {
        let psk_c = riot_sys::psk_params_t {
            key: riot_sys::credman_buffer_t {
                s: psk.key.as_ptr() as *const c_void,
                len: psk.key.len() as size_t,
            },
            id: riot_sys::credman_buffer_t {
                s: psk.id.as_ptr() as *const c_void,
                len: psk.id.len() as size_t,
            },
            hint: riot_sys::credman_buffer_t {
                s: psk.hint.as_ptr() as *const c_void,
                len: psk.hint.len() as size_t,
            },
        };
        Credential {
            credential: riot_sys::credman_credential_t {
                type_: riot_sys::credman_type_t_CREDMAN_TYPE_PSK,
                tag,
                params: riot_sys::credman_credential_t__bindgen_ty_1 { psk: psk_c },
            },
            params: Params::Psk(psk),
        }
    }

    pub fn new_ecdsa(tag: CredmanTag, ecdsa: EcdsaParams<'a>) -> Self {
        let ecdsa_c = riot_sys::ecdsa_params_t {
            private_key: ecdsa.private_key.as_ptr() as *const c_void,
            public_key: riot_sys::ecdsa_public_key_t {
                x: ecdsa.public_key.x.as_ptr() as *const c_void,
                y: ecdsa.public_key.y.as_ptr() as *const c_void,
            },
            client_keys: ecdsa.client_keys.as_ptr() as *mut _, //Is assumed not to be mutated during the runtime?
            client_keys_size: ecdsa.client_keys.len() as size_t,
        };

        Credential {
            credential: riot_sys::credman_credential_t {
                type_: riot_sys::credman_type_t_CREDMAN_TYPE_ECDSA,
                tag,
                params: riot_sys::credman_credential_t__bindgen_ty_1 { ecdsa: ecdsa_c },
            },
            params: Params::Ecdsa(ecdsa),
        }
    }
}

pub fn scope<Main, RMain>(credential: &Credential, main: Main) -> Result<RMain, CredmanStatus>
where
    Main: FnOnce() -> RMain,
{
    let status = credman_add(credential);
    if matches!(status, CredmanStatus::CredmanOK) {
        return Err(status);
    }
    let ret = main();
    unsafe {
        riot_sys::credman_delete(credential.credential.tag, credential.credential.type_);
    }
    Ok(ret)
}

// int credman_add(const credman_credential_t *credential);
fn credman_add(credential: &Credential) -> CredmanStatus {
    let res = unsafe {
        riot_sys::credman_add(&credential.credential as *const riot_sys::credman_credential_t)
    };
    CredmanStatus::from_value(res)
}

// int credman_get(credman_credential_t *credential, credman_tag_t tag, credman_type_t type);
pub fn credman_get(tag: CredmanTag, typ: CredmanType) -> Result<CredentialRef, CredmanStatus> {
    //TOD: CredmanCredRef zurückgeben
    let mut cred: riot_sys::credman_credential_t = Default::default();
    let res = unsafe {
        riot_sys::credman_get(
            &mut cred as *mut riot_sys::credman_credential_t,
            tag,
            CredmanType::to_c(typ),
        )
    };
    match CredmanStatus::from_value(res) {
        CredmanStatus::CredmanOK => Ok(CredentialRef { credential: cred }),
        status => Err(status),
    }
}

// int credman_get_used_count(void);
pub fn credman_get_used_count() -> u32 {
    unsafe { riot_sys::credman_get_used_count() as u32 }
}
