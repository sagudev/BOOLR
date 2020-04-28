use wasm_bindgen::prelude::*;
use web_sys::OscillatorType;

#[wasm_bindgen]
pub fn beep(frequency: Option<f32>, duration: Option<i32>) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let ctx = web_sys::AudioContext::new()?;
    let osc = ctx.create_oscillator()?;
    osc.set_type(OscillatorType::Sine);
    osc.frequency().set_value(frequency.unwrap_or(440.0));
    osc.connect_with_audio_node(&ctx.destination())?;
    osc.start()?;
    // create fn from it
    window.set_timeout_with_callback_and_timeout_and_arguments_0(osc.stop(), duration.unwrap_or(500));
    //std::thread::sleep(std::time::Duration::from_millis(duration.unwrap_or(500)));// hm
    //osc.stop()?;
    Ok(())
}
