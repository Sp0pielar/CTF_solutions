#!allow(non_snake_case_functions)

pub const AES_BLOCKLEN : usize = 16;
pub const AES_KEYLEN : usize = 16;
pub const AES_KEYEXPSIZE : usize = 176;

#[allow(non_snake_case)]
#[repr(C)]
pub struct AES_ctx {
    pub RoundKey: [u8; AES_KEYEXPSIZE],
    pub Iv:       [u8; AES_BLOCKLEN]
}

#[link(name = "aes")]
extern "C" {
    pub fn AES_init_ctx(
        ctx: *mut AES_ctx,
        key: *const u8
    );

    pub fn AES_ECB_encrypt(
        ctx: *const AES_ctx,
        buf: *mut u8
    );

    pub fn AES_ECB_decrypt(
        ctx: *const AES_ctx,
        buf: *mut u8
    );

}

#[allow(dead_code)]
pub fn init_aes(ctx: &mut AES_ctx , key: &[u8;16] ){
    unsafe {
        let pkey = key.as_ptr();
        AES_init_ctx(ctx, pkey);
    }
}

#[allow(dead_code)]
pub fn encrypt_aes_ecb(ctx: &AES_ctx , buf: &mut[u8;16] ){
    unsafe {
        let pbuf = buf.as_mut_ptr();
        AES_ECB_encrypt(ctx, pbuf);
    }
}

#[allow(dead_code)]
pub fn decrypt_aes_ecb(ctx: &AES_ctx , buf: &mut[u8;16] ){
    unsafe {
        let pbuf = buf.as_mut_ptr();
        AES_ECB_decrypt(ctx, pbuf);
    }
}
