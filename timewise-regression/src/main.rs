use timewise_regression::{regressao_linear, calcular_r2, calcular_mse, prever};

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.1, 6.0, 8.1, 10.2];

    match regressao_linear(&x, &y) {
        Ok(coef) => {
            println!("Coeficientes: a = {:.4}, b = {:.4}", coef.a, coef.b);
            println!("R² = {:.4}", calcular_r2(&x, &y, &coef));
            println!("MSE = {:.4}", calcular_mse(&x, &y, &coef));
            println!("Previsão para x = 6.0: {:.4}", prever(6.0, &coef));
        }
        Err(e) => println!("Erro: {}", e),
    }
}