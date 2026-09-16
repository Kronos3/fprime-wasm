#![no_std]
#![no_main]

use example::*;
use fprime_core::*;

#[fprime_main]
pub fn main() {
    set_fail_mode(FailMode::Permissive);

    CdhCore.cmdDisp.CMD_NO_OP();
    CdhCore.cmdDisp.CMD_NO_OP_STRING("STRINGS");
    CdhCore.events.SET_EVENT_FILTER(ACTIVITY_HI, DISABLED);

    CdhCore.health.HLTH_PING_ENABLE("asd", ENABLED);

    // let (ev_dropped, _) = CdhCore.events.EventsDropped();
    // if ev_dropped > 2 {
    //     CdhCore.cmdDisp.CMD_NO_OP_STRING("DROPPED 2");
    // }

    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");

    Ref.dpDemo.Dp(ASYNC, 2, PROC_TYPE_ONE);

    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.SelectColor(BLUE);
    Ref.dpDemo.Dp(IMMEDIATE, 2, PROC_TYPE_TWO);

    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.wasmSeq.LOAD("helloworld");
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);

    Ref.dpDemo.SelectColor(GREEN);
}
