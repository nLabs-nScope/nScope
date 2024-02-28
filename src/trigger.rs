use neon::prelude::*;
use nscope::{Trigger, TriggerType};

use crate::{JsNscopeHandle, Objectify};

impl Objectify for Trigger {
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let is_on = cx.boolean(self.is_enabled);
        obj.set(cx, "isOn", is_on)?;

        let source = cx.string(format!("Ch{}", self.source_channel + 1));
        obj.set(cx, "source", source)?;

        let level = cx.number(self.trigger_level);
        obj.set(cx, "level", level)?;

        let trigger_type = match self.trigger_type {
            TriggerType::RisingEdge => { cx.string("RisingEdge") }
            TriggerType::FallingEdge => { cx.string("FallingEdge") }
        };
        obj.set(cx, "type", trigger_type)?;

        Ok(obj)
    }
}

pub fn get_trigger_status(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let nscope_handle = js_nscope_handle.borrow();

    let obj = nscope_handle.trigger.to_object(&mut cx)?;

    let mut disable_ui = cx.boolean(true);

    if let Some(nscope) = &nscope_handle.device {
        if nscope.is_connected() {
            disable_ui = cx.boolean(false);
        }
    }

    obj.set(&mut cx, "uiDisabled", disable_ui)?;

    Ok(obj)
}

pub fn set_trigger_on(mut cx: FunctionContext) -> JsResult<JsNull> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let is_on = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    let mut nscope_handle = js_nscope_handle.borrow_mut();
    nscope_handle.trigger.is_enabled = is_on;
    Ok(cx.null())
}

pub fn set_trigger_source(mut cx: FunctionContext) -> JsResult<JsNull> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let source = cx.argument::<JsString>(1)?.value(&mut cx);
    let mut nscope_handle = js_nscope_handle.borrow_mut();

    nscope_handle.trigger.source_channel = match source.as_str() {
        "Ch1" => 0,
        "Ch2" => 1,
        "Ch3" => 2,
        "Ch4" => 3,
        _ => panic!("Invalid channel string"),
    };
    Ok(cx.null())
}

pub fn set_trigger_delay(mut cx: FunctionContext) -> JsResult<JsNull> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let delay = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let mut nscope_handle = js_nscope_handle.borrow_mut();
    nscope_handle.trigger.trigger_delay_us = delay as u32;
    Ok(cx.null())
}

pub fn set_trigger_level(mut cx: FunctionContext) -> JsResult<JsNull> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let mut nscope_handle = js_nscope_handle.borrow_mut();
    nscope_handle.trigger.trigger_level = level;
    Ok(cx.null())
}

pub fn set_trigger_type(mut cx: FunctionContext) -> JsResult<JsNull> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let trigger_type = cx.argument::<JsString>(1)?.value(&mut cx);
    let mut nscope_handle = js_nscope_handle.borrow_mut();
    nscope_handle.trigger.trigger_type = match trigger_type.as_str() {
        "RisingEdge" => TriggerType::RisingEdge,
        "FallingEdge" => TriggerType::FallingEdge,
        _ => panic!("Invalid trigger type string"),
    };
    Ok(cx.null())
}