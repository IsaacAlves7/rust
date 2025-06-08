<a href="https://github.com/IsaacAlves7/rust"><img src="https://github.com/user-attachments/assets/33490bda-7577-41e1-9711-8df466750fa8"></a>

> ⚙️🦀 **Preparação**: Para este conteúdo, o aluno deverá dispor de um computador com acesso à internet, um web browser com suporte a HTML 5 (Google Chrome, Mozilla Firefox, Microsoft Edge, Safari, Opera etc.), um editor de texto ou IDE (VSCode etc.) e o software Rust, com a versão mais recente, instalado na sua máquina local.

# It's a repository of Rust programming ⚙️🦀
<a href="https://www.rust-lang.org/"><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" align="right" height="77"></a>

**Rust** é uma linguagem de programação compilada, multiparadigma, desenvolvida inicialmente pela Mozilla Research. Ela combina o desempenho de linguagens como C/C++ com garantias de segurança de memória sem precisar de garbage collector. É projetada para ser "segura, concorrente e prática", mas diferente de outras linguagens seguras, Rust não usa coletor de lixo. Possui suporte nativo ao WebAssembly.

A linguagem apareceu como um projeto pessoal de Graydon Hoare, empregado da Mozilla. A organização começou a apoiar o projeto em 2009 e anunciou-o em 2010. No mesmo ano, os esforços mudaram do compilador original (escrito em OCaml) para um auto-hospedado feito em Rust. Conhecido por rustc, conseguiu compilar-se pela primeira vez em 2011 e utiliza o LLVM como back-end. Foi lançada pela primeira vez uma versão numerada pré-alfa em 2012. Rust 1.0, a primeira versão estável, foi lançada em 15 de maio de 2015.

Foi considerada pelo público a linguagem "mais amada" por nove anos consecutivos, de acordo com pesquisas conduzidas pelo site Stack Overflow de 2016 a 2024, e está entre as 25 linguagens mais populares, de acordo com pesquisas conduzidas pela RedMonk desde 2018.

Rust e Go (Golang) compartilham algumas semelhanças conceituais, mas suas filosofias de design, propósitos e escolhas técnicas são bastante diferentes.

Diferenças marcantes entre Go e Rust:

| Aspecto                      | Go                                        | Rust                                                            |
| ---------------------------- | ----------------------------------------- | --------------------------------------------------------------- |
| **Gerenciamento de memória** | Coletor de lixo (GC)                      | Ownership e borrowing sem GC                                    |
| **Concorrência**             | Simples, nativa com goroutines e channels | Mais complexo, mas seguro (async/await, threads, `tokio`)       |
| **Curva de aprendizado**     | Baixa: fácil para iniciantes              | Alta: exige mais entendimento técnico                           |
| **Orientação a objetos**     | Estruturas simples com métodos            | Traits e generics poderosos                                     |
| **Erros**                    | Erros são tratados com valores retornados | Sistema de tipos com `Result` e `Option`, forçando o tratamento |
| **Tempo de compilação**      | Rápido                                    | Mais lento                                                      |
| **Comunidade corporativa**   | Google, Uber, Dropbox                     | Mozilla, AWS, Microsoft                                         |

<img src="https://github.com/user-attachments/assets/fed87b8d-7fd8-4988-91b1-1d80b9e46f0d" align="right" height="77">

O "crab" (caranguejo em inglês) é uma referência ao mascote da linguagem Rust, chamado **Ferris** que é um caranguejo simpático, criado pela comunidade Rust, e foi adotado como símbolo não-oficial da linguagem. Às vezes, a comunidade se refere aos desenvolvedores Rust como "crabby" ou fala coisas como "c**rust**acean coding", mas isso é só brincadeira.

Porque em inglês, o termo "rust" (ferrugem) remete a ferrugem de ferro, e "ferris" é um trocadilho com "ferrous" (ferroso). A ideia era ter algo fofo, memorável e relacionado a ferrugem — daí veio o caranguejo Ferris, com suas patinhas representando segurança e controle (características centrais do Rust).

Verificando a instalação correta do gerenciador de pacotes da linguagem Rust:

```sh
cargo --version
```

Verificando a instalação correta do compilador da linguagem Rust:

```sh
rustc --version
```

Iniciando o diretório: Após a criação do diretório `main`

```sh
cargo init
```

## [Rust] Hello, World!
