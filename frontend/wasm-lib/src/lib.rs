use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Airdrop {
    address: String,
    staked: String,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wasm_send() -> Result<JsValue, JsValue> {

    let dummy_data = vec![Airdrop{address: String::from("osmo1mkply7ymvvdk56aamsdftqux2pvws9e6nqfchy"), staked: String::from("69")},
    Airdrop{address: String::from("osmo1wr3jreg7vrmwlp7ylccshgrv2n53grdfv33hqe"), staked: String::from("420")}];

    Ok(serde_wasm_bindgen::to_value(&dummy_data)?)
} 

#[wasm_bindgen]
pub fn wasmg_get(val: JsValue) -> Result<(), JsValue> {
    let csv_data: Vec<Airdrop> = serde_wasm_bindgen::from_value(val)?;
    alert(&format!("{:#?}", csv_data));
    Ok(())
}