#![no_std]
#![no_main]

mod dictionary;
pub use dictionary::*;

use crate::Defs::Fw::DpCfg::ProcType;
use crate::Defs::Ref::DpDemo::DpReqType;
use crate::Defs::Svc::EventManager::{Enabled, FilterSeverity};

use fprime_core::*;

#[panic_handler]
fn __panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Some(msg) = info.message().as_str() {
        message(EventSeverity::WarningHi, msg);
    } else {
        message(EventSeverity::WarningHi, "rust panic")
    }

    panic(PanicCode::RustPanic);
}

#[unsafe(no_mangle)]
pub fn main() {
    CdhCore.cmdDisp.CMD_NO_OP();
    CdhCore
        .events
        .SET_EVENT_FILTER(FilterSeverity::ACTIVITY_HI, Enabled::DISABLED);

    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo
        .Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo
        .Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo
        .Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo
        .Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo
        .Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo
        .Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
}
