use std::collections::LinkedList;

use controller::{function::Function, metodos::{Metodos, EulerResult, EulerMejoradoResult, RK4Result}};
use serde::{Serialize, Deserialize};

pub mod controller;

#[derive(Serialize, Deserialize)]
pub struct CalculationResult {
    pub euler: Option<LinkedList<EulerResult>>,
    pub euler_mejorado: Option<LinkedList<EulerMejoradoResult>>,
    pub runge_kutta: Option<LinkedList<RK4Result>>,
}

#[tauri::command]
fn calcular(func: &str, x0: &str, y0: &str, h: &str, x_final: &str) -> Result<String, String> {
    let func = match Function::new(&func) {
        Ok(func) => func,
        Err(_) => return Err("Función no válida".to_string()),
    };

    let x0 = x0.parse::<f64>().map_err(|_| "Valor X0 no válido".to_string())?;
    let y0 = y0.parse::<f64>().map_err(|_| "Valor Y0 no válido".to_string())?;
    let h = h.parse::<f64>().map_err(|_| "Valor h no válido".to_string())?;
    let x_final = x_final.parse::<f64>().map_err(|_| "Valor X final no válido".to_string())?;

    let euler_result = Metodos::euler(&func, &x0, &y0, &h, &x_final).map_err(|e| e.to_string())?;
    let euler_mejorado_result = Metodos::euler_mejorado(&func, &x0, &y0, &h, &x_final).map_err(|e| e.to_string())?;
    let runge_kutta_result = Metodos::runger_kutta4(&func, &x0, &y0, &h, &x_final).map_err(|e| e.to_string())?;

    /*
    Ok(
        CalculationResult {
            euler: Some(euler_result),
            euler_mejorado: Some(euler_mejorado_result),
            runge_kutta: Some(runge_kutta_result),
        }
    )
     */
    let mut json = String::new();
    json.push_str("{\"euler\":");
    json.push_str(&euler_result);
    json.push_str(",\n");

    json.push_str("\"euler_mejorado\":");
    json.push_str(&euler_mejorado_result);
    json.push_str(",\n");

    json.push_str("\"rk4\":");
    json.push_str(&runge_kutta_result);
    json.push_str("\n}");

    Ok(json)
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calcular])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
