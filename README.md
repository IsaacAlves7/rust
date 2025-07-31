<a href="https://github.com/IsaacAlves7/rust"><img src="https://github.com/user-attachments/assets/33490bda-7577-41e1-9711-8df466750fa8"></a>

> ⚙️🦀 **Preparação**: Para este conteúdo, o aluno deverá dispor de um computador com acesso à internet, um web browser com suporte a HTML 5 (Google Chrome, Mozilla Firefox, Microsoft Edge, Safari, Opera etc.), um editor de texto ou IDE (VSCode etc.) e o software Rust, com a versão mais recente, instalado na sua máquina local.

Sou um especialista em desenvolvimento de software com foco no ecossistema Rust, utilizando a linguagem para construir sistemas altamente seguros, performáticos e confiáveis. Tenho experiência sólida em boas práticas como SOLID, DRY, KISS, TDA e SoC, aplicação de design patterns (criacionais, estruturais e comportamentais), além da adoção de princípios de Clean Code e Clean Architecture para garantir legibilidade, manutenibilidade e escalabilidade do código.

No front-end, atuo em projetos com SPAs (React, Angular, Vue), SSR (Next.js) e design responsivo com HTML/CSS, Flexbox e CSS Grid, baseados em protótipos do Figma (UI/UX).

No back-end, desenvolvo APIs REST e GraphQL utilizando frameworks como Actix Web, Axum e Juniper, com foco em segurança, concorrência segura e alto desempenho. Aplico conceitos como DDD e Arquitetura Hexagonal, aproveitando o sistema de tipos avançado e o borrow checker do Rust para garantir segurança em tempo de compilação. Utilizo crates como Serde para serialização, Tokio para programação assíncrona e Diesel ou SQLx para integração com bancos relacionais.

Tenho domínio de Git e colaboração com pipelines de CI/CD (GitHub Actions, GitLab CI), automação com Cargo e Clippy para linting, além de testes unitários, de integração e de propriedade com crates como proptest e criterion para benchmarks.

Em observabilidade, utilizo OpenTelemetry, Prometheus e Grafana para monitoramento, e crates como tracing e log para logging estruturado. Em arquiteturas distribuídas ou baseadas em microsserviços, integro mensageria com Kafka, RabbitMQ ou NATS, mantendo foco em comunicação eficiente e robustez.

Também implemento ferramentas de analytics como Google Analytics 4 ou soluções personalizadas para análise de uso em produção. Tenho experiência com entrega de software em ambientes PaaS, uso de recursos em nuvem (AWS, GCP, Azure) e infraestrutura como código com Docker e Kubernetes.

# It's a repository of Rust programming ⚙️🦀
<a href="https://www.rust-lang.org/"><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" align="right" height="77"></a>

O **Rust** é uma linguagem de programação compilada, multiparadigma, desenvolvida inicialmente pela Mozilla Research. Ela combina o desempenho de linguagens como C/C++ com garantias de segurança de memória sem precisar de garbage collector. É projetada para ser "segura, concorrente e prática", mas diferente de outras linguagens seguras, Rust não usa coletor de lixo e possui suporte nativo ao <a href="">WebAssembly</a>.

A linguagem apareceu como um projeto pessoal de Graydon Hoare, empregado da Mozilla. A organização começou a apoiar o projeto em 2009 e anunciou-o em 2010. No mesmo ano, os esforços mudaram do compilador original (escrito em OCaml) para um auto-hospedado feito em Rust. Conhecido por rustc, conseguiu compilar-se pela primeira vez em 2011 e utiliza o LLVM como back-end. Foi lançada pela primeira vez uma versão numerada pré-alfa em 2012. Rust 1.0, a primeira versão estável, foi lançada em 15 de maio de 2015.

Foi considerada pelo público a linguagem "mais amada" por nove anos consecutivos, de acordo com pesquisas conduzidas pelo site Stack Overflow de 2016 a 2024, e está entre as 25 linguagens mais populares, de acordo com pesquisas conduzidas pela RedMonk desde 2018.

Rust, Go (Golang) e Elixir são linguagens modernas que surgiram para resolver problemas distintos do desenvolvimento de software, mas compartilham algumas ideias em termos de desempenho, concorrência e segurança. Abaixo, segue uma análise comparativa destacando semelhanças e diferenças entre elas:

- **Desempenho alto**: Ambas as linguagens são compiladas para código nativo, o que significa que entregam desempenho comparável ao C/C++. São ótimas escolhas para sistemas de baixa latência, alta performance e que exigem controle de recursos.

- **Segurança na memória**: Tanto Rust quanto Go evitam falhas comuns em C/C++ como segfaults, uso de ponteiros inválidos e data races (corridas de dados), embora o façam de formas diferentes: Go usa coletor de lixo (garbage collector), já Rust usa ownership e borrowing com verificação em tempo de compilação — sem GC.

- **Foco em concorrência**: Go é conhecido pelas `goroutines` e `channels`, que facilitam a escrita de código concorrente. Rust tem um sistema de concorrência segura por design, evitando data races com checagens em tempo de compilação. Usa threads, async/await e crates como `tokio`.

- **Simplicidade de uso comparado a C/C++**: Ambas foram projetadas como alternativas modernas às linguagens de baixo nível como C, oferecendo mais segurança, produtividade e melhores ferramentas.

Diferenças marcantes entre Go, Rust e Elixir:

| **Aspecto**                  | **Go**                                          | **Rust**                                                           | **Elixir**                                                                   |
| ---------------------------- | ----------------------------------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------- |
| **Gerenciamento de memória** | Coletor de lixo (GC)                            | Ownership e borrowing sem GC                                       | Garbage collector (BEAM VM)                                                  |
| **Concorrência**             | Simples, nativa com goroutines e channels       | Complexa mas segura (threads, async/await, `tokio`)                | Altamente concorrente via Actor Model (processos leves do BEAM)              |
| **Curva de aprendizado**     | Baixa: sintaxe direta e fácil                   | Alta: exige domínio de sistema de tipos e memória                  | Média: exige entender paradigma funcional e arquitetura OTP                  |
| **Paradigma principal**      | Imperativo e concorrente (multiparadigma)       | Sistemas, baixo nível, concorrente e seguro                        | Funcional, concorrente e reativo                                             |
| **Orientação a objetos**     | Structs com métodos (sem classes)               | Traits e generics poderosos (sem herança clássica)                 | Sem OO tradicional, usa módulos, funções e mensagens                         |
| **Tratamento de erros**      | Erros como valores retornados (`if err != nil`) | `Result<T, E>` e `Option<T>`, exige tratamento explícito           | `try/rescue`, `with`, *let it crash* (filosofia resiliente)                  |
| **Tempo de compilação**      | Rápido                                          | Mais lento, mas com verificações rigorosas                         | Não compilado estaticamente (interpretado na BEAM VM)                        |
| **Desempenho**               | Muito bom, próximo ao C em muitos casos         | Altíssimo, excelente para sistemas críticos e baixo nível          | Bom, mas menos focado em CPU-bound, mais em escalabilidade                   |
| **Escalabilidade**           | Boa, com goroutines leves                       | Boa, com controle manual de threads ou async                       | Excelente, nativamente distribuído com suporte a sistemas massivos           |
| **Comunidade corporativa**   | Google, Uber, Dropbox, Cloudflare               | Mozilla, AWS, Microsoft, Discord                                   | Plataformas como WhatsApp, Discord (usando Erlang/Elixir), PagerDuty         |
| **Melhor para...**           | APIs, CLIs, microserviços, redes                | Sistemas embarcados, web de alta performance, engines, blockchains | Sistemas tolerantes a falha, comunicação em tempo real, back-ends escaláveis |

Quando escolher Go ou Rust ou Elixir? Veja abaixo o ecossistema e casos de uso:

| Linguagem  | Casos de Uso Comuns                                             |
| ---------- | --------------------------------------------------------------- |
| **Rust**   | Sistemas embarcados, CLI, WebAssembly, engines, bancos de dados |
| **Go**     | Microserviços, APIs, ferramentas de rede, DevOps, cloud         |
| **Elixir** | Chat, mensageria, telecom, sistemas distribuídos, IoT           |

Use Go se você quer:

- Produtividade rápida
- Concorrência simples (ex: servidores web, microserviços)
- Facilidade de leitura e manutenção por equipes grandes
- Curva de aprendizado mais suave

Use Rust se você precisa de:

- Máximo controle e segurança sem coletor de lixo
- Programação de sistemas, drivers, engines de jogos, ou aplicações críticas de performance
- Alta segurança de memória e concorrência complexa
- Zero runtime e performance máxima

<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2024"><img src="https://github.com/user-attachments/assets/fed87b8d-7fd8-4988-91b1-1d80b9e46f0d" align="right" height="77"></a>

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
A função `fn main()` não recebe nenhum valor de entrada, visto que os parênteses estão vazios, ela está retornando um tipo de dado chamado `unity`, que é um tipo vazio.

[![main.rs](https://img.shields.io/badge/-main.rs-E6C3A5?style=social&logo=rust&logoColor=E6C3A5)](#)

```rust
fn main() {
    println!("Hello, world!");
}
```

# 🧪 [Rust] DDD, BDD e TDD
**DDD (Domain-Driven Design)**, **TDD (Test-Driven Development)** e **BDD (Behavior-Driven Development)** **podem ser aplicados em Rust**, embora a forma como você os pratica difere de linguagens orientadas a objetos ou funcionais como Elixir. Vamos ver como cada um se encaixa no ecossistema Rust:

DDD (Domain-Driven Design) em Rust, Embora Rust **não tenha orientação a objetos clássica**, ele **suporta modelagem rica de domínios** com `struct`, `enum`, `trait` e módulos.

Como aplicar DDD em Rust:

* **Entidades** → `struct` com identidade (ID persistente).
* **Value Objects** → `struct` imutáveis, sem identidade própria.
* **Serviços de Domínio** → funções puras ou `trait impl` com lógica de negócio.
* **Repositórios** → traits e structs que isolam a persistência (simulados com banco em memória ou SQLite/Postgres).
* **Agregados** → structs que agrupam entidades/VOs com regras de consistência.
* **Módulos** → separação de bounded contexts.

Exemplo simples:

```rust
pub struct User {
    pub id: u32,
    pub name: String,
}

impl User {
    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}

pub struct RegisterUserService;

impl RegisterUserService {
    pub fn register(name: &str) -> User {
        User { id: rand::random(), name: name.to_string() }
    }
}
```

TDD (Test-Driven Development) em Rust, Rust tem suporte nativo a testes via o framework embutido `#[cfg(test)]`.

* Você pode escrever testes antes do código, e o compilador garante a correção e segurança.
* Os testes são rápidos, seguros (com borrowing/ownership) e evitam falhas em tempo de execução.

Exemplo com TDD:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

BDD (Behavior-Driven Development) em Rust, o BDD **não é nativo**, mas pode ser implementado com frameworks de terceiros como:

* [`cucumber-rs`](https://crates.io/crates/cucumber) → permite escrever testes Gherkin (`.feature`) como no Cucumber.
* [`speculate`](https://crates.io/crates/speculate) → sintaxe alternativa ao estilo BDD (mais próxima ao RSpec).

Exemplo com `cucumber-rs`:

```gherkin
Feature: Login
  Scenario: Successful login
    Given a registered user
    When they provide valid credentials
    Then they are logged in
```

Em Rust, você escreve os *steps* em código com `async` e `Result`.

Resumo comparativo no contexto de Rust

| Conceito | Aplicação em Rust                                                                           |
| -------- | ------------------------------------------------------------------------------------------- |
| **DDD**  | Possível com `struct`, `trait`, `mod`; foco em modelagem com tipos seguros e explícitos.    |
| **TDD**  | Suporte nativo (`#[test]`), altamente eficiente com compilação rápida e segura.             |
| **BDD**  | Usável com frameworks como `cucumber-rs`, mas menos comum na prática em comparação com TDD. |
