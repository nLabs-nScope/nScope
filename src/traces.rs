use std::sync::mpsc::TryRecvError;

use neon::prelude::*;

use crate::{JsNscopeHandle, NscopeTraces, RunState};

impl NscopeTraces {
    pub(crate) fn clear(&mut self) {
        for s in &mut self.samples {
            s.clear()
        }
        self.current_head = 0;
    }

    fn initialize_trace_object(&self, cx: &mut FunctionContext, trace_data: Handle<JsObject>) {
        let x_data = JsArray::new(cx, nscope::Sample::num_channels() as usize);
        let y_data = JsArray::new(cx, nscope::Sample::num_channels() as usize);

        for ch in 0u32..nscope::Sample::num_channels() + 1 {
            let empty_array = JsArray::new(cx, self.num_samples);
            x_data.set(cx, ch, empty_array).unwrap();
            for idx in 0usize..self.num_samples {
                let x_array: Handle<JsArray> = x_data.get(cx, ch as u32).unwrap();
                let t = cx.number(idx as f64 * 12.0 / self.num_samples as f64);
                x_array.set(cx, idx as u32, t).unwrap();
            }
            let empty_array = JsArray::new(cx, self.num_samples);
            y_data.set(cx, ch, empty_array).unwrap();
        }

        for (idx, sample) in self.samples.iter().enumerate() {
            for ch in 0usize..nscope::Sample::num_channels() as usize {
                let x_array: Handle<JsArray> = x_data.get(cx, ch as u32).unwrap();
                let t = cx.number(idx as f64 * 12.0 / self.num_samples as f64);
                x_array.set(cx, idx as u32, t).unwrap();

                if let Some(data) = sample.data[ch] {
                    let y_array: Handle<JsArray> = y_data.get(cx, ch as u32).unwrap();
                    let y = cx.number(data);
                    y_array.set(cx, idx as u32, y).unwrap();
                }
            }
        }

        trace_data.set(cx, "x", x_data).unwrap();
        trace_data.set(cx, "y", y_data).unwrap();
    }
}

fn set_trace_y(cx: &mut FunctionContext, trace_data: Handle<JsObject>, sample: &nscope::Sample, idx: usize) {
    let y_data: Handle<JsArray> = trace_data.get(cx, "y").unwrap();

    for ch in 0usize..nscope::Sample::num_channels() as usize {
        let y_array: Handle<JsArray> = y_data.get(cx, ch as u32).unwrap();
        if let Some(data) = sample.data[ch] {
            let y = cx.number(data);
            y_array.set(cx, idx as u32, y).unwrap();
        } else {
            let y = cx.null();
            y_array.set(cx, idx as u32, y).unwrap();
        }
    }
}

fn clear_trace_y(cx: &mut FunctionContext, trace_data: Handle<JsObject>, idx: usize) {
    let y_data: Handle<JsArray> = trace_data.get(cx, "y").unwrap();

    for ch in 0usize..nscope::Sample::num_channels() as usize {
        let y_array: Handle<JsArray> = y_data.get(cx, ch as u32).unwrap();
        let y = cx.null();
        y_array.set(cx, idx as u32, y).unwrap();
    }
}


pub fn get_traces(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_nscope_handle = cx.argument::<JsNscopeHandle>(0)?;
    let trace_data = cx.argument::<JsObject>(1)?;
    let mut nscope_handle = js_nscope_handle.borrow_mut();

    if nscope_handle.device.is_none() {
        return Ok(cx.empty_object());
    }

    if !nscope_handle.get_device().is_connected() {
        return Ok(cx.empty_object());
    }


    // If we have no ongoing sweep
    if nscope_handle.sweep_handle.is_none() {
        if nscope_handle.run_state != RunState::Stopped {

            // Make a new request
            nscope_handle.sweep_handle = Some(nscope_handle.get_device().request(
                nscope_handle.sample_rate,
                nscope_handle.traces.num_samples as u32,
                Some(nscope_handle.trigger),
            ));
            nscope_handle.traces.current_head = 0;
            for idx in 0..nscope_handle.traces.trace_gap() {
                nscope_handle.traces.samples[idx].clear();
            }

            nscope_handle.traces.initialize_trace_object(&mut cx, trace_data);
        }
    }

    if nscope_handle.sweep_handle.is_some() {
        loop {
            match nscope_handle.receiver().try_recv() {
                Ok(sample) => {
                    let idx = nscope_handle.traces.current_head;
                    let trace_gap = nscope_handle.traces.trace_gap();

                    set_trace_y(&mut cx, trace_data, &sample, idx);
                    nscope_handle.traces.samples[idx] = sample;

                    if idx + trace_gap < nscope_handle.traces.samples.len() {
                        clear_trace_y(&mut cx, trace_data, idx + trace_gap);
                        nscope_handle.traces.samples[idx + trace_gap].clear();
                    }
                    nscope_handle.traces.current_head += 1;
                }
                Err(TryRecvError::Empty) => { break; }
                Err(TryRecvError::Disconnected) => {

                    // Check to see if we've completed a single sweep
                    if nscope_handle.traces.num_samples == nscope_handle.traces.current_head {
                        // If we've ended a stream and we're single, this should have stopped.
                        if nscope_handle.run_state == RunState::Single {
                            nscope_handle.run_state = RunState::Stopped;
                        }
                    }
                    nscope_handle.sweep_handle = None;
                    break;
                }
            }
        }
    }
    Ok(trace_data)
}