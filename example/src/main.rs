#![no_std]
#![no_main]

use crate::dictionary::{
    Fw::DpCfg::ProcType,
    Ref::DpDemo::DpReqType::{self, IMMEDIATE},
};
use fprime_core::*;

mod dictionary;
use dictionary::*;

#[fprime_main]
pub fn main() {
    CdhCore.cmdDisp.CMD_NO_OP();
    CdhCore.cmdDisp.CMD_NO_OP_STRING("STRINGS");
    CdhCore.events.SET_EVENT_FILTER(
        Svc::EventManager::FilterSeverity::ACTIVITY_HI,
        Svc::EventManager::Enabled::DISABLED,
    );

    let (ev_dropped, _) = CdhCore.events.EventsDropped();
    if ev_dropped > 2 {
        CdhCore.cmdDisp.CMD_NO_OP_STRING("DROPPED 2");
    }

    Ref.dpDemo.Dp(IMMEDIATE, 0, ProcType::PROC_TYPE_NONE);
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
