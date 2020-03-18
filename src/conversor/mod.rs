mod constantes;

use crate::tipo::TipoExtenso;
use constantes::*;

pub fn gerar_extenso(inteiro: String, decimal: String, tipo: TipoExtenso) -> String {
    let inteiro_extenso = extenso(inteiro);
    let decimal_extenso = extenso(decimal);

    match tipo {
        TipoExtenso::DECIMAL => mascara_decimal(inteiro_extenso, decimal_extenso),
        TipoExtenso::PORCENTAGEM => mascara_porcetagem(inteiro_extenso, decimal_extenso),
        TipoExtenso::MONETARIO => mascara_monetario(inteiro_extenso, decimal_extenso),
    }
}

fn mascara_decimal(inteiro: String, decimal: String) -> String {
    if decimal != "" {
        format!("{} ponto {}", inteiro, decimal)
    } else {
        inteiro
    }
}

fn mascara_porcetagem(inteiro: String, decimal: String) -> String {
    if decimal != "" {
        format!("{} ponto {} por cento", inteiro, decimal)
    } else {
        format!("{} por cento", inteiro)
    }
}

fn mascara_monetario(inteiro: String, decimal: String) -> String {
    if decimal != "" {
        format!("{} reais e {} centavos", inteiro, decimal)
    } else {
        format!("{} reais", inteiro)
    }
}

fn extenso(valor: String) -> String {
    let mut extenso = String::new();

    let quantidade_centena = ((valor.len() as f32) / 3.0).round() as i32;

    for contador_centena in 0..quantidade_centena {
        let tripla: String = valor
            .chars()
            .skip((contador_centena * 3) as usize)
            .take(3)
            .collect();
        let tripla_numero: i32 = tripla.parse().unwrap();
        let unidades_centena: Vec<&str> = tripla.split("").collect();

        let mut centena = CENTENAS[unidades_centena[1].parse::<usize>().unwrap()];
        let dezena = DEZENAS[unidades_centena[2].parse::<usize>().unwrap()];
        let dezena_composta = DEZENAS_COMPOSTAS[unidades_centena[3].parse::<usize>().unwrap()];
        let mut unidade = UNIDADES[unidades_centena[3].parse::<usize>().unwrap()];
        let casa = CASAS[(quantidade_centena - contador_centena - 1) as usize];
        let casa_plural = CASAS_PLURAL[(quantidade_centena - contador_centena - 1) as usize];

        if tripla_numero == 1 {
            unidade = "";
        }

        if tripla_numero >= 100 && tripla_numero < 200 {
            centena = "cento";
        }

        if centena != "" {
            extenso.push_str(centena);
            if unidade != "" || dezena != "" {
                extenso.push_str(" e ");
            }
        }

        if dezena == "dez" {
            extenso.push_str(dezena_composta);
        } else {
            if dezena != "" {
                extenso.push_str(dezena);
                if unidade != "" {
                    extenso.push_str(" e ");
                }
            }

            if unidade != "" {
                extenso.push_str(unidade);
            }
        }

        if casa != "" && casa_plural != "" {
            extenso.push_str(" ");

            if tripla_numero > 1 {
                extenso.push_str(casa_plural);
            } else {
                extenso.push_str(casa);
            }
            extenso.push_str(", ");
        }
    }

    extenso
}
