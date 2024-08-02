use super::function::Function;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::LinkedList;

#[derive(Serialize, Deserialize)]
pub struct EulerResult{
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct EulerMejoradoResult{
    pub x: f64,
    pub y: f64,
    pub y_star: f64,
}

#[derive(Serialize, Deserialize)]
pub struct RK4Result{
    pub x: f64,
    pub y: f64,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub k4: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NewtonRapsonResult{
    pub x: f64,
    pub err: String,
}

pub struct Metodos{}

impl Metodos {
    pub fn euler(func:&Function, x0: &f64, y0: &f64, h: &f64, x_final: &f64) -> Result<String,Box<dyn Error>>{
        let mut list = LinkedList::new();
        let mut x = *x0;
        let mut y = *y0;

        while x < *x_final {
            y = y + h * func.eval(&x, &y)?;
            x+=h;
            list.push_back(EulerResult{x, y});
        }

        let json = match serde_json::to_string(&list)  {
            Ok(json) => json,
            Err(_) =>{
                let mut list = LinkedList::new();
                list.push_back(EulerResult{x: 0., y: 0.});
                serde_json::to_string(&list).unwrap()
            },
        };

        Ok(json)
    }

    pub fn euler_mejorado(func:&Function, x0: &f64, y0: &f64, h: &f64, x_final: &f64) -> Result<String,Box<dyn Error>>{
        let mut list = LinkedList::new();
        let mut x = *x0;
        let mut y = *y0;
        let mut y_star;

        while x < *x_final {
            y_star = y + h * func.eval(&x, &y)?;
            y += (h / 2.0) * (func.eval(&x, &y)? + func.eval(&(x+h), &y_star)?);
            x += h;
            list.push_back(EulerMejoradoResult{x, y, y_star});
        }
        

        let json = match serde_json::to_string(&list)  {
            Ok(json) => json,
            Err(_) =>{
                let mut list = LinkedList::new();
                list.push_back(EulerMejoradoResult{x: 0., y: 0., y_star: 0.});
                serde_json::to_string(&list).unwrap()
            },
        };

        Ok(json)
    }

    pub fn runger_kutta4(func:&Function, x0: &f64, y0: &f64, h: &f64, x_final: &f64) -> Result<String,Box<dyn Error>>{
        let mut list = LinkedList::new();
        let mut x = *x0;
        let mut y = *y0;

        while x < *x_final {
            let k1 = func.eval(&x, &y)?;
            let k2 = func.eval(&(x + 0.5 * h), &(y + 0.5 * h * k1))?;
            let k3 = func.eval(&(x + 0.5 * h), &(y + 0.5 * h * k2))?;
            let k4 = func.eval(&(x + h), &(y + h * k3))?;
            y = y + (h/6.) * (k1 + 2.*k2 + 2.*k3 + k4);
            x+=h;
            list.push_back(RK4Result{x, y, k1, k2, k3, k4});
        }

        let json = match serde_json::to_string(&list)  {
            Ok(json) => json,
            Err(_) =>{
                let mut list = LinkedList::new();
                list.push_back(RK4Result{x: 0., y: 0., k1: 0., k2: 0., k3:0., k4:0.});
                serde_json::to_string(&list).unwrap()
            },
        };

        println!("{json}");
        Ok(json)
    }

    pub fn newton_rapson(func: &Function, deriv: &Function, x0: &f64, err_limit: &f64) -> Result<String, String> {
        let mut list = LinkedList::new();   
        let mut x = x0;
        let mut err = 100.;

        list.push_back(NewtonRapsonResult{x: *x0, err: String::new()});

        let json = match serde_json::to_string(&list)  {
            Ok(json) => json,
            Err(_) =>{
                let mut list = LinkedList::new();
                list.push_back(NewtonRapsonResult{x: *x0, err: String::new()});
                serde_json::to_string(&list).unwrap()
            },
        };

        println!("{json}");
        Ok(json)
    }
}
