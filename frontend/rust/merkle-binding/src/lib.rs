use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::mem;
use std::io;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Airdrop {
    address: String,
    staked: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stakers {
    address: String,
    staked: f64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wasm_sends() -> Result<JsValue, JsValue> {

    let dummy_data = vec![Airdrop{address: String::from("osmo1mkply7ymvvdk56aamsdftqux2pvws9e6nqfchy"), staked: String::from("69")},
    Airdrop{address: String::from("osmo1wr3jreg7vrmwlp7ylccshgrv2n53grdfv33hqe"), staked: String::from("420")}];

    Ok(serde_wasm_bindgen::to_value(&dummy_data)?)
} 

#[wasm_bindgen]
pub fn wasm_gets(val: JsValue) -> Result<(), JsValue> {
    let csv_data: Vec<Airdrop> = serde_wasm_bindgen::from_value(val)?;
    let new_data = csv_data.clone();

    let mut total_staked : f64 = 0.0;

    for result in csv_data {
        let record: Airdrop = result;
        total_staked += record.staked.parse::<f64>().unwrap();
    }

    let total_stakers = mem::size_of::<Airdrop>();
    let  multiplier : f64 = (total_stakers as f64) / (total_staked as f64);

    let mut wtr = csv::Writer::from_writer(io::stdout());
    
    // let mut check_subdenoms = vec![Airdrop{address: String::from("osmo1mkply7ymvvdk56aamsdftqux2pvws9e6nqfchy"), staked: String::from("69")},
    // Airdrop{address: String::from("osmo1wr3jreg7vrmwlp7ylccshgrv2n53grdfv33hqe"), staked: String::from("420")}];


    for result in new_data {
        let mut record: Airdrop = result;
        record.staked = ((record.staked.parse::<f64>().unwrap() as f64) * multiplier * 5000000.0).to_string();
        wtr.serialize(&record);
        // dummy_data.push(record);
    }


    // alert(&format!("{:#?}", total_staked));
    // alert(&format!("{:#?}", total_stakers));
    // alert(&format!("{:#?}",  multiplier));
    // console_log!("{:#?}", wtr);
    // console_log!("{:#?}", check_subdenoms);


    Ok(())
}