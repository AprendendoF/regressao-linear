# 📈 TimeWise Regression - Regressão Linear Pura em Rust

Projeto desenvolvido para a disciplina de Programação de Sistemas, focado na **implementação pura de regressão linear em Rust**, sem uso de bibliotecas externas.

> Este módulo simula parte da ferramenta interna de análise de séries temporais da startup fictícia **TimeWise Analytics**.

---

## 📌 Objetivo

O projeto tem como objetivo:

- Implementar uma regressão linear simples em Rust a partir de uma série temporal;
- Avaliar o modelo utilizando métricas como **R²** (coeficiente de determinação) e **MSE** (erro quadrático médio);
- Realizar **previsões futuras** com base nos coeficientes calculados;
- Garantir robustez e confiabilidade por meio de **testes unitários**.

---

## 🛠️ Funcionalidades

| Função                  | Descrição                                                                |
|-------------------------|--------------------------------------------------------------------------|
| `regressao_linear`      | Calcula os coeficientes da reta (y = ax + b) a partir dos dados          |
| `calcular_r2`           | Calcula o coeficiente de determinação R²                                 |
| `calcular_mse`          | Calcula o erro quadrático médio (MSE)                                    |
| `prever`                | Faz previsões para novos valores `x` usando os coeficientes calculados   |

---

## 🧪 Testes

O projeto inclui **testes unitários** que verificam:

- Correção da regressão linear;
- Cálculo correto das métricas R² e MSE;
- Funcionamento da função de previsão;
- Tratamento de entradas inválidas (vetores de tamanhos diferentes ou vazios).

> Os prints dos testes podem ser gerados no terminal após rodar `cargo test`.

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
kotlin
Copiar código
timewise-regression/
├── src/
│   ├── lib.rs          # Lógica principal (funções e regressão)
│   └── main.rs         # Exemplo de execução
├── Cargo.toml          # Manifesto do projeto
├── README.md           # Este arquivo
└── testes.pdf          # Prints dos testes unitários (opcional)
🤔 Limitações
O modelo implementado é univariado (apenas uma variável independente);

Não considera padrões sazonais, autocorrelação ou não linearidades;

Ideal para séries temporais simples com tendência linear.

🧠 Estratégias Adotadas
Implementação manual dos cálculos estatísticos, sem crates externas;

Testes unitários para garantir robustez e confiabilidade;

Código modularizado e comentado, facilitando manutenção e futuras melhorias;

Validação de entradas inválidas para evitar erros de execução.

👨‍💻 Autor
Breno Ferreira
📅 Junho de 2025
📘 Projeto individual – Regressão Linear
Instituição: Análise e Desenvolvimento de Sistemas (ADS)
