fn numero_por_extenso<T: Into<f64>>(_valor: T) -> String {
    let mut string = String::new();
    let _casas = ["", "mil", "milhão", "bilhão", "trilhão", "quatrilhão"];
    let _centenas = ["", "cem", "duzentos", "trezentos", "quatrocentos","quinhentos", "seiscentos", "setecentos", "oitocentos", "novecentos"];
    let _dezenas = ["", "dez", "vinte", "trinta", "quarenta", "cinquenta","sessenta", "setenta", "oitenta", "noventa"];
    let _dezenas_compostas = ["dez", "onze", "doze", "treze", "quatorze", "quinze","dezesseis", "dezesete", "dezoito", "dezenove"];
    let _unidades = ["", "um", "dois", "três", "quatro", "cinco", "seis","sete", "oito", "nove"];
    
    let _valor_string = _valor.into().to_string();
    let _separar_decimal: Vec<&str> = _valor_string.split(".").collect();

    let _inteiro = normalizar(String::from(_separar_decimal[0]));
    let mut _decimal = String::from("");

    if _separar_decimal.len() > 1 {
        _decimal = String::from(_separar_decimal[1]);
    }

    let quantidade_centena = ((_inteiro.len() as f32) / 3.0).round() as i32;
    
    for contador_centena in (1..quantidade_centena+1) {
        println!("{}", contador_centena);
        // println!("{}", &_casas[contador_centena..contador_centena+1]);
        // println!("{}", &_inteiro[inicio..inicio+1]);
    }

    string.push_str(_separar_decimal[0]);

    string
}

fn normalizar(valor: String) -> String {
    if valor.len() % 3 == 0 {
        valor.to_string()
    } else {
        normalizar(format!("{}{}", String::from("0"), valor))
    }
}

fn main() {
    println!("{}", numero_por_extenso(1234));
}
