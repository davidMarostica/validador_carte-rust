pub fn validar_cpf(cpf: &str) -> bool {
    let numeros: String = cpf.chars().filter(|c| c.is_digit(10)).collect();

    if numeros.len() != 11 {
        return false;
    }

    if numeros.chars().all(|c| c == numeros.chars().next().unwrap()) {
        return false;
    }

    let digitos: Vec<u32> = numeros.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut soma = 0;
    for i in 0..9 {
        soma += digitos[i] * (10 - i as u32);
    }
    let resto = soma % 11;
    let dv1 = if resto < 2 { 0 } else { 11 - resto };

    soma = 0;
    for i in 0..10 {
        soma += digitos[i] * (11 - i as u32);
    }
    let resto = soma % 11;
    let dv2 = if resto < 2 { 0 } else { 11 - resto };

    dv1 == digitos[9] && dv2 == digitos[10]
}
