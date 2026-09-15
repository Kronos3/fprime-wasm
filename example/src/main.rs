#![no_std]
#![no_main]

use fprime_core::*;
mod dictionary;
use dictionary::*;

use crate::Defs::Fw::DpCfg::ProcType;
use crate::Defs::Ref::DpDemo::DpReqType;
use crate::Defs::Svc::EventManager::{Enabled, FilterSeverity};

#[fprime_main]
pub fn main() {
    CdhCore.cmdDisp.CMD_NO_OP();
    CdhCore.cmdDisp.CMD_NO_OP_STRING("STRINGS");
    CdhCore
        .events
        .SET_EVENT_FILTER(FilterSeverity::ACTIVITY_HI, Enabled::DISABLED);

    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(DpReqType::IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
}
