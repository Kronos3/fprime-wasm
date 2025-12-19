use crate::types::type_name;
use crate::util::{annotate, hex_literal, qualified_identifier, NameKind};
use crate::Qualifier;
use proc_macro2::TokenStream;
use quote::quote;

pub fn telemetry_channel(tlm: &fprime_dictionary::TelemetryChannel) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&tlm.name, NameKind::Function);
    let ty = type_name(&tlm.type_name);

    let id = hex_literal(tlm.id);

    let def = quote! {
        pub fn #name() -> FprimeResult<(#ty, crate::fw::TimeValue)> {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            let mut value_buf: [u8; <#ty as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            unsafe {
                sys::telemetry(#id, &mut time_buf, &mut value_buf)
            }?;

            Ok((<#ty as Serializable>::deserialize(value_buf), crate::fw::TimeValue::deserialize(time_buf)))
        }
    };

    (q, annotate(def, &tlm.annotation))
}
