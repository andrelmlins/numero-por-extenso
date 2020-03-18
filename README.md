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

## Licença

Numero por Extenso é um software Open Source [licenciado pelo MIT](https://github.com/andrelmlins/numero-por-extenso/blob/master/LICENSE).
