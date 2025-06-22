# 📈 TimeWise Regression - Regressão Linear Pura em Rust

Projeto desenvolvido para a disciplina de Programação de Sistemas com foco na **implementação pura de regressão linear em Rust**, sem uso de bibliotecas externas.

> 🚀 Este módulo simula parte da ferramenta interna da startup fictícia **TimeWise Analytics**, especializada em análise e previsão de séries temporais.

---

## 📌 Objetivo

Desenvolver uma solução eficiente em Rust para:

- Calcular uma regressão linear simples a partir de uma série temporal;
- Avaliar o modelo utilizando métricas como **R²** e **MSE**;
- Realizar **previsões futuras** com base no modelo gerado;
- Garantir qualidade e segurança com **testes unitários** e documentação clara.

---

## 🛠️ Funcionalidades Implementadas

| Função                  | Descrição                                                                |
|-------------------------|--------------------------------------------------------------------------|
| `regressao_linear`      | Calcula os coeficientes da reta (y = ax + b) com base nos dados          |
| `calcular_r2`           | Calcula o coeficiente de determinação R²                                 |
| `calcular_mse`          | Calcula o erro quadrático médio (MSE)                                    |
| `prever`                | Realiza previsão para um novo valor `x` usando os coeficientes gerados   |

---

## 🧪 Testes

Todos os módulos foram validados por **testes unitários**:

- ✅ Regressão linear correta
- ✅ Cálculo preciso de R² e MSE
- ✅ Previsão funcional
- ✅ Tratamento de vetores inválidos

> Prints dos testes unitários bem-sucedidos estão disponíveis no arquivo `testes.pdf`.

---

## 📊 Exemplo de Uso

```rust
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
📂 Estrutura do Projeto
bash
Copiar
Editar
timewise-regression/
├── src/
│   ├── lib.rs          # Lógica principal
│   └── main.rs         # Exemplo de execução
├── Cargo.toml          # Manifesto do projeto
├── README.md           # Este arquivo
└── testes.pdf          # Prints dos testes unitários
🤔 Limitações
O modelo é simples: regressão linear univariada;

Não lida com sazonalidades, autocorrelação ou dados não lineares;

Ideal para padrões lineares simples em séries temporais.

🎥 Demonstração em Vídeo
📽️ Veja a explicação do projeto e da lógica aplicada no seguinte link:

🔗 YouTube - TimeWise Regression (explicação)

🧠 Estratégias Adotadas
Implementação 100% manual dos cálculos estatísticos;

Uso de testes para garantir robustez contra entradas inválidas;

Código modularizado e comentado para facilitar manutenção e extensão futura;

Evitado uso de crates externos para seguir as exigências de "regressão linear pura".

👨‍💻 Autor
Breno Ferreira
📅 Junho de 2025
📘 Projeto individual – Regressão Linear
Instituição: Análise e Desenvolvimento de Sistemas (ADS)
