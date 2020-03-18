mod constantes;

use crate::constantes::*;

pub fn numero_por_extenso<T: Into<f64>>(_valor: T) -> String {
    let mut string = String::new();
    let _valor_string = _valor.into().to_string();
    let _separar_decimal: Vec<&str> = _valor_string.split(".").collect();

    let _inteiro = normalizar(String::from(_separar_decimal[0]));
    let mut _decimal = String::from("");

    if _separar_decimal.len() > 1 {
        _decimal = String::from(_separar_decimal[1]);
    }

    let quantidade_centena = ((_inteiro.len() as f32) / 3.0).round() as i32;

    for contador_centena in 0..quantidade_centena {
        let tripla: String = _inteiro
            .chars()
            .skip((contador_centena * 3) as usize)
            .take(3)
            .collect();
        let tripla_numero: i32 = tripla.parse().unwrap();
        let unidades_centena: Vec<&str> = tripla.split("").collect();

        let centena = CENTENAS[unidades_centena[1].parse::<usize>().unwrap()];
        let dezena = DEZENAS[unidades_centena[2].parse::<usize>().unwrap()];
        let dezena_composta = DEZENAS_COMPOSTAS[unidades_centena[3].parse::<usize>().unwrap()];
        let mut unidade = UNIDADES[unidades_centena[3].parse::<usize>().unwrap()];
        let casa = CASAS[(quantidade_centena - contador_centena - 1) as usize];
        let casa_plural = CASAS_PLURAL[(quantidade_centena - contador_centena - 1) as usize];

        if tripla_numero == 1 {
            unidade = "";
        }

        if centena != "" {
            string.push_str(centena);
            string.push_str(" e ");
        }

        if dezena == "dez" {
            string.push_str(dezena_composta);
        } else {
            if dezena != "" {
                string.push_str(dezena);
                string.push_str(" e ");
            }

            if unidade != "" {
                string.push_str(unidade);
            }
        }

        if casa != "" && casa_plural != "" {
            string.push_str(" ");

            if tripla_numero > 1 {
                string.push_str(casa_plural);
            } else {
                string.push_str(casa);
            }
            string.push_str(", ");
        }
    }

    string
}

fn normalizar(valor: String) -> String {
    if valor.len() % 3 == 0 {
        valor.to_string()
    } else {
        normalizar(format!("{}{}", String::from("0"), valor))
    }
}
