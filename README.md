# Número por Extenso

Biblioteca Rust para conversão por extenso de valores numéricos.

[![Crates.io](https://img.shields.io/crates/v/numero-por-extenso)](https://crates.io/crates/numero-por-extenso) &bull; [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/andrelmlins/numero-por-extenso/blob/master/LICENSE) &bull; [![Build Status](https://travis-ci.com/andrelmlins/numero-por-extenso.svg?branch=master)](https://travis-ci.com/andrelmlins/numero-por-extenso)

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

use numero_por_extenso::*;

println!("numero por extenso: {}", numero_por_extenso(123456));
```

## Licença

Numero por Extenso é um software Open Source [licenciado pelo MIT](https://github.com/andrelmlins/numero-por-extenso/blob/master/LICENSE).
