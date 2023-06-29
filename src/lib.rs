mod utils;

use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

use base64::{engine::general_purpose, Engine as _};
use hpke_rs::prelude::*;
use hpke_rs_crypto::types::{AeadAlgorithm, KdfAlgorithm, KemAlgorithm};
use hpke_rs_rust_crypto::HpkeRustCrypto;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hpke!");
}

#[wasm_bindgen]
pub fn seal(pub_key: String, message: String) -> String {
    let hpke = Hpke::<HpkeRustCrypto>::new(
        HpkeMode::Base,
        KemAlgorithm::DhKem25519,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
    );
    let pk_r = general_purpose::STANDARD.decode(pub_key).unwrap();
    let info = b"";
    let aad = b"";

    let (enc, ctxt) = hpke
        .seal(
            &pk_r.into(),
            info,
            aad,
            message.as_bytes(),
            None,
            None,
            None,
        )
        .unwrap();

    format!(
        "{}|{}",
        general_purpose::STANDARD.encode(enc),
        general_purpose::STANDARD.encode(ctxt)
    )
}
