// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

#[tauri::command]
fn calc_newton(func: &str, derivada: &str, x0: &str, err_limit: &str) ->Result<String,String>{
    let func = match Function::new(&func) {
        Ok(func) => func,
        Err(_) => return Err("Función no válida".to_string()),
    };

    let derivada = match Function::new(&derivada) {
        Ok(func) => func,
        Err(_) => return Err("Función no válida".to_string()),
    };

    let x0 = x0.parse::<f64>().map_err(|_| "Valor X0 no válido".to_string())?;
    let err_limit = err_limit.parse::<f64>().map_err(|_| "Valor X0 no válido".to_string())?;
    let newton_rapson_res:String = Metodos::newton_rapson(&func, &derivada, &x0, &err_limit).map_err(|e| e.to_string())?;
    
    println!("a");
    let mut json = String::new();
    json.push_str("\"newton_rapson\":");
    json.push_str(&newton_rapson_res);
    json.push_str("\n}");

    Ok(json)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calcular])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
