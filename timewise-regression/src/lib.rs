/// Representa os coeficientes de uma regressão linear (y = a * x + b)
#[derive(Debug, PartialEq)]
pub struct Coeficientes {
    pub a: f64,
    pub b: f64,
}

/// Calcula os coeficientes da regressão linear a partir de vetores de entrada x e y
pub fn regressao_linear(x: &[f64], y: &[f64]) -> Result<Coeficientes, String> {
    let n = x.len();
    if n != y.len() || n == 0 {
        return Err("Vetores inválidos".to_string());
    }

    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let media_x = soma_x / n as f64;
    let media_y = soma_y / n as f64;

    let numerador = soma_xy - n as f64 * media_x * media_y;
    let denominador = soma_x2 - n as f64 * media_x * media_x;

    if denominador == 0.0 {
        return Err("Divisão por zero".to_string());
    }

    let a = numerador / denominador;
    let b = media_y - a * media_x;

    Ok(Coeficientes { a, b })
}

/// Calcula o R² (coeficiente de determinação)
pub fn calcular_r2(x: &[f64], y: &[f64], coef: &Coeficientes) -> f64 {
    let media_y: f64 = y.iter().sum::<f64>() / y.len() as f64;

    let ss_total: f64 = y.iter().map(|yi| (yi - media_y).powi(2)).sum();
    let ss_residual: f64 = x.iter()
        .zip(y)
        .map(|(xi, yi)| (yi - (coef.a * xi + coef.b)).powi(2))
        .sum();

    1.0 - (ss_residual / ss_total)
}

/// Calcula o erro quadrático médio (MSE)
pub fn calcular_mse(x: &[f64], y: &[f64], coef: &Coeficientes) -> f64 {
    let n = x.len();
    let soma_erros: f64 = x.iter()
        .zip(y)
        .map(|(xi, yi)| (yi - (coef.a * xi + coef.b)).powi(2))
        .sum();
    soma_erros / n as f64
}

/// Faz previsões com base nos coeficientes calculados
pub fn prever(x: f64, coef: &Coeficientes) -> f64 {
    coef.a * x + coef.b
}