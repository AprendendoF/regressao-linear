#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regressao_linear() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let coef = regressao_linear(&x, &y).unwrap();
        assert!((coef.a - 2.0).abs() < 1e-6);
        assert!((coef.b - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_mse() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let coef = regressao_linear(&x, &y).unwrap();
        let mse = calcular_mse(&x, &y, &coef);
        assert!(mse < 1e-6);
    }

    #[test]
    fn test_r2() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let coef = regressao_linear(&x, &y).unwrap();
        let r2 = calcular_r2(&x, &y, &coef);
        assert!((r2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_previsao() {
        let coef = Coeficientes { a: 2.0, b: 1.0 };
        let resultado = prever(4.0, &coef);
        assert!((resultado - 9.0).abs() < 1e-6);
    }

    #[test]
    fn test_erro_vetores_invalidos() {
        let x = vec![1.0, 2.0];
        let y = vec![2.0];
        let resultado = regressao_linear(&x, &y);
        assert!(resultado.is_err());
    }
}
