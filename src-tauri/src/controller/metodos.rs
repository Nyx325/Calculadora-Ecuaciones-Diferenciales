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

pub struct Metodos{}

#[allow(dead_code, unused)]
impl Metodos {
    pub fn euler(func:&Function, x0: &f64, y0: &f64, h: &f64, x_final: &f64) -> Result<LinkedList<EulerResult>,Box<dyn Error>>{
        let mut list = LinkedList::new();
        let mut x = *x0;
        let mut y = *y0;

        println!("METODO EULER\nXn\tYn\t");
        while(x < *x_final){
            y = y + h * func.eval(&x, &y)?;
            x+=h;
            list.push_back(EulerResult{x, y});
            println!("{:.2}\t{:.4}\t", x, y)
        }
        Ok(list)
    }

    pub fn euler_mejorado(func:&Function, x0: &f64, y0: &f64, h: &f64, x_final: &f64) -> Result<LinkedList<EulerMejoradoResult>,Box<dyn Error>>{
        let mut list = LinkedList::new();
        let mut x = *x0;
        let mut y = *y0;
        let mut y_star;

        println!("EULER MEJORADO\nx0\ty0\ty0*");

        while (x < *x_final) {
            y_star = y + h * func.eval(&x, &y)?;
            y += (h / 2.0) * (func.eval(&x, &y)? + func.eval(&(x+h), &y_star)?);
            x += h;
            list.push_back(EulerMejoradoResult{x, y, y_star});
            println!("{:.2}\t{:.4}\t{:.4}", x, y, y_star)
        }
        Ok(list)
    }

    pub fn runger_kutta4(func:&Function, x0: &f64, y0: &f64, h: &f64, x_final: &f64) -> Result<LinkedList<RK4Result>,Box<dyn Error>>{
        let mut list = LinkedList::new();
        let mut x = *x0;
        let mut y = *y0;

        println!("RUNGER KUTTA ORDEN 4");
        println!("Xn\tYn\tK1\tK2\tK3\tK4");
        while(x < *x_final){
            let k1 = func.eval(&x, &y)?;
            let k2 = func.eval(&(x + 0.5 * h), &(y + 0.5 * h * k1))?;
            let k3 = func.eval(&(x + 0.5 * h), &(y + 0.5 * h * k2))?;
            let k4 = func.eval(&(x + h), &(y + h * k3))?;
            y = y + (h/6.) * (k1 + 2.*k2 + 2.*k3 + k4);
            x+=h;
            list.push_back(RK4Result{x, y, k1, k2, k3, k4});
            println!("{:.2}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}", x, y, k1, k2, k3, k4);
        }

        Ok(list)
    }
}
