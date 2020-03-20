/*!
# Número por Extenso

Biblioteca Rust para conversão por extenso de valores numéricos.

[![Crates.io](https://img.shields.io/crates/v/numero-por-extenso)](https://crates.io/crates/numero-por-extenso) &bull; [![Crates.io](https://img.shields.io/crates/l/numero-por-extenso)](https://github.com/andrelmlins/numero-por-extenso/blob/master/LICENSE) &bull; [![Build Status](https://travis-ci.com/andrelmlins/numero-por-extenso.svg?branch=master)](https://travis-ci.com/andrelmlins/numero-por-extenso)

## Instalação

Adicione isso no seu `Cargo.toml`:

```toml
[dependencies]
numero-por-extenso = "0.1.0"
```

## Uso básico

Abaixo uma forma simples de como utilizar essa biblioteca:

```rust
extern crate numero_por_extenso;

use numero_por_extenso::por_extenso;
use numero_por_extenso::TipoExtenso;

println!("numero por extenso: {}", por_extenso(123456, TipoExtenso::DECIMAL));
// cento e vinte e três mil, quatrocentos e cinquenta e seis

println!("numero por extenso: {}", por_extenso(1234.56, TipoExtenso::MONETARIO));
// mil, duzentos e trinta e quatro reais e cinquenta e seis centavos

println!("numero por extenso: {}", por_extenso(23, TipoExtenso::PORCENTAGEM));
// vinte e três por cento
```
*/

mod conversor;
mod helpers;
mod tipo;

use helpers::*;
pub use tipo::TipoExtenso;

pub fn por_extenso<T: Into<f64>>(valor: T, tipo: TipoExtenso) -> String {
    let _valor_string = valor.into().to_string();
    let _separar_decimal: Vec<&str> = _valor_string.split(".").collect();

    let _inteiro = normalizar(String::from(_separar_decimal[0]));
    let mut _decimal = String::from("");

    if _separar_decimal.len() > 1 {
        _decimal = normalizar(String::from(_separar_decimal[1]));
    }

    let string = conversor::gerar_extenso(_inteiro, _decimal, tipo);

    string
}
