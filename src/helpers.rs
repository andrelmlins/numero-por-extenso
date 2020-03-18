pub fn normalizar(valor: String) -> String {
    if valor.len() % 3 == 0 {
        valor.to_string()
    } else {
        normalizar(format!("{}{}", String::from("0"), valor))
    }
}
