mod conversor;
mod helpers;
mod tipo;

use helpers::*;
pub use tipo::Tipo;

pub fn numero_por_extenso<T: Into<f64>>(_valor: T, _tipo: Tipo) -> String {
    let _valor_string = _valor.into().to_string();
    let _separar_decimal: Vec<&str> = _valor_string.split(".").collect();

    let _inteiro = normalizar(String::from(_separar_decimal[0]));
    let mut _decimal = String::from("");

    if _separar_decimal.len() > 1 {
        _decimal = String::from(_separar_decimal[1]);
    }

    let string = conversor::gerar_extenso(_inteiro, _decimal, _tipo);

    string
}

fn main() {
    println!("{}", numero_por_extenso(226.40, Tipo::DECIMAL))
}
