use crate::tree::Qualifier;
use crate::types::type_name;
use crate::util::{annotate, hex_literal, split_identifier};
use proc_macro2::TokenStream;
use quote::quote;

pub fn telemetry_channel(tlm: &fprime_dictionary::TelemetryChannel) -> (Qualifier, TokenStream) {
    let (q, name) = split_identifier(&tlm.name);
    let ty = type_name(&tlm.type_name);

    let id = hex_literal(tlm.id);

    let def = quote! {
        pub fn #name(&self) -> (#ty, super::Defs::Fw::TimeValue) {
            let mut time_buf: [u8; super::Defs::Fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            let mut value_buf: [u8; <#ty as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            unsafe {
                telemetry(#id, &mut time_buf, &mut value_buf)
            };

            (
                <#ty as Serializable>::deserialize(value_buf),
                super::Defs::Fw::TimeValue::deserialize(time_buf)
            )
        }
    };

    (q, annotate(def, &tlm.annotation))
}
