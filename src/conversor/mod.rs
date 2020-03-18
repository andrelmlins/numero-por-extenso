mod constantes;

use crate::tipo::Tipo;
use constantes::*;

pub fn gerar_extenso(inteiro: String, decimal: String, tipo: Tipo) -> String {
    let inteiroExtenso = extenso(inteiro);
    let decimalExtenso = extenso(decimal);

    match tipo {
        Tipo::DECIMAL => decimal(inteiroExtenso, decimalExtenso),
        Tipo::PORCENTAGEM => porcetagem(inteiroExtenso, decimalExtenso),
        Tipo::MONETARIO => monetario(inteiroExtenso, decimalExtenso),
    }
}

fn decimal(inteiro: String, decimal: String) -> String {
    format!("{} ponto {}", inteiro, decimal)
}

fn porcetagem(inteiro: String, decimal: String) -> String {
    format!("{} ponto {} por cento", inteiro, decimal)
}

fn monetario(inteiro: String, decimal: String) -> String {
    format!("{} reais e {} centavos", inteiro, decimal)
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
