# Validador de CPF em Rust

Esta é uma aplicação simples em Rust para validar CPF brasileiro. O projeto fornece uma função para verificar se um CPF é válido, seguindo as regras de cálculo dos dígitos verificadores.

## Funcionalidades

- Validação de CPF com base nas regras oficiais do cálculo dos dígitos verificadores
- Aceita CPFs com ou sem pontuação, como pontos, traços e espaços
- Implementação leve e fácil de integrar em outros projetos Rust

## Estrutura do projeto

- `validador/` - crate principal do projeto
- `validador/src/lib.rs` - implementação da função de validação

## Como usar

1. Entre na pasta do projeto:

   ```bash
   cd validador
   ```

2. Execute os testes:

   ```bash
   cargo test
   ```

3. Exemplo de uso em Rust:

   ```rust
   use validador::validar_cpf;

   fn main() {
       let cpf = "529.982.247-25";
       println!("{}", validar_cpf(cpf));
   }
   ```

## Exemplo de saída

- Para um CPF válido, a função retorna `true`
- Para um CPF inválido, a função retorna `false`

## Contato

- Nome: David Aparecido da Silva
- Contato: davidmarosticasilvasilva25@gmail.com
- LinkedIn: https://www.linkedin.com/in/david-aparecido-da-silva
