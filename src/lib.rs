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
