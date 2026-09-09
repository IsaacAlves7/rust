<a href="https://github.com/IsaacAlves7/rust"><img src="https://github.com/user-attachments/assets/33490bda-7577-41e1-9711-8df466750fa8"></a>

> Versículo chave: "Consagre ao Senhor tudo o que você faz, e os seus planos serão bem-sucedidos." - Provérbios 16:3

https://jsoverson.medium.com/was-rust-worth-it-f43d171fb1b3

Sou um especialista em desenvolvimento de software com foco no ecossistema Rust, utilizando a linguagem para construir sistemas altamente seguros, performáticos e confiáveis. Tenho experiência sólida em boas práticas como SOLID, DRY, KISS, TDA e SoC, aplicação de design patterns (criacionais, estruturais e comportamentais), além da adoção de princípios de Clean Code e Clean Architecture para garantir legibilidade, manutenibilidade e escalabilidade do código.

No front-end, atuo em projetos com SPAs (React, Angular, Vue), SSR (Next.js) e design responsivo com HTML/CSS, Flexbox e CSS Grid, baseados em protótipos do Figma (UI/UX).

No back-end, desenvolvo APIs REST e GraphQL utilizando frameworks como Actix Web, Axum e Juniper, com foco em segurança, concorrência segura e alto desempenho. Aplico conceitos como DDD e Arquitetura Hexagonal, aproveitando o sistema de tipos avançado e o borrow checker do Rust para garantir segurança em tempo de compilação. Utilizo crates como Serde para serialização, Tokio para programação assíncrona e Diesel ou SQLx para integração com bancos relacionais.

Tenho domínio de Git e colaboração com pipelines de CI/CD (GitHub Actions, GitLab CI), automação com Cargo e Clippy para linting, além de testes unitários, de integração e de propriedade com crates como proptest e criterion para benchmarks.

Em observabilidade, utilizo OpenTelemetry, Prometheus e Grafana para monitoramento, e crates como tracing e log para logging estruturado. Em arquiteturas distribuídas ou baseadas em microsserviços, integro mensageria com Kafka, RabbitMQ ou NATS, mantendo foco em comunicação eficiente e robustez.

Também implemento ferramentas de analytics como Google Analytics 4 ou soluções personalizadas para análise de uso em produção. Tenho experiência com entrega de software em ambientes PaaS, uso de recursos em nuvem (AWS, GCP, Azure) e infraestrutura como código com Docker e Kubernetes.

# It's a repository of Rust programming ⚙️🦀
> ⚙️🦀 **Preparação**: Para este conteúdo, o aluno deverá dispor de um computador com acesso à internet, um web browser com suporte a HTML 5 (Google Chrome, Mozilla Firefox, Microsoft Edge, Safari, Opera etc.), um editor de texto ou IDE (VSCode etc.) e o software Rust, com a versão mais recente, instalado na sua máquina local.

<a href="https://www.rust-lang.org/"><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" align="right" height="77"></a>

O **Rust** é uma linguagem de programação compilada, multiparadigma, desenvolvida inicialmente pela Mozilla Research. Ela combina o desempenho de linguagens como C/C++ com garantias de segurança de memória sem precisar de garbage collector. É projetada para ser "segura, concorrente e prática", mas diferente de outras linguagens seguras, Rust não usa coletor de lixo e possui suporte nativo ao <a href="">WebAssembly</a>.

A linguagem apareceu como um projeto pessoal de Graydon Hoare, empregado da Mozilla. A organização começou a apoiar o projeto em 2009 e anunciou-o em 2010. No mesmo ano, os esforços mudaram do compilador original (escrito em OCaml) para um auto-hospedado feito em Rust. Conhecido por rustc, conseguiu compilar-se pela primeira vez em 2011 e utiliza o LLVM como back-end. Foi lançada pela primeira vez uma versão numerada pré-alfa em 2012. Rust 1.0, a primeira versão estável, foi lançada em 15 de maio de 2015.

Foi considerada pelo público a linguagem "mais amada" por nove anos consecutivos, de acordo com pesquisas conduzidas pelo site Stack Overflow de 2016 a 2024, e está entre as 25 linguagens mais populares, de acordo com pesquisas conduzidas pela RedMonk desde 2018.

Rust, Go (Golang) e Elixir são linguagens modernas que surgiram para resolver problemas distintos do desenvolvimento de software, mas compartilham algumas ideias em termos de desempenho, concorrência e segurança. Abaixo, segue uma análise comparativa destacando semelhanças e diferenças entre elas:

- **Desempenho alto** (Performance): Ambas as linguagens são compiladas para código nativo, o que significa que entregam desempenho comparável ao C/C++. São ótimas escolhas para sistemas de baixa latência, alta performance e que exigem controle de recursos.

- **Segurança na memória** (Memory-Safe): Tanto Rust quanto Go evitam falhas comuns em C/C++ como segfaults, uso de ponteiros inválidos e data races (corridas de dados), embora o façam de formas diferentes: Go usa coletor de lixo (garbage collector), já Rust usa ownership e borrowing com verificação em tempo de compilação — sem GC.

- **Foco em concorrência** (Concurrency): Go é conhecido pelas `goroutines` e `channels`, que facilitam a escrita de código concorrente. Rust tem um sistema de concorrência segura por design, evitando data races com checagens em tempo de compilação. Usa threads, async/await e crates como `tokio`.

- **Simplicidade de uso comparado a C/C++**: Ambas foram projetadas como alternativas modernas às linguagens de baixo nível como C, oferecendo mais segurança, produtividade e melhores ferramentas.

![c75872149c8ea0036f0e03f0ddee7f5a2c9fedf0](https://github.com/user-attachments/assets/ac8f3300-3def-4829-a981-352ecb50d79b)

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
| **Rust**   | Sistemas embarcados, OS kernels, CLI, WebAssembly, engines, bancos de dados, web servers |
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

Para rodar o Rust no console do terminal: Esse comando usa o compilador do Rust (`rustc`) para transformar seu arquivo-fonte (`exemplo.rs`) em um binário executável.

```sh
rustc exemplo.rs
```

## [Rust] Hello, World!
Iniciando o diretório: Após a criação do arquivo `main`, pode notar a semelhança da estrutura do diretório com as de outras linguagens de programação modernas como JavaScript iniciado através do Node.js.

```sh
cargo new main
```

Iniciando: Iniciando o diretório: Após a criação do diretório `main`

```sh
cargo init
```

template:

```
|- src
| |
| |- main.rs
| |- target
| |- .gitignore 
| |- cargo.lock
| |- cargo.toml
```

A função `fn main()` não recebe nenhum valor de entrada, visto que os parênteses estão vazios, ela está retornando um tipo de dado chamado `unity`, que é um tipo vazio.

[![main.rs](https://img.shields.io/badge/-main.rs-E6C3A5?style=social&logo=rust&logoColor=E6C3A5)](#)

```rust
// Função
fn main() {
    println('Hello, world!') // macro
}
```

Iniciando: Para rodar o projeto:

```sh
cargo run
# Hello, world!
```

Para checar a compilação:

```sh
cargo check
```

## [Rust] Comentários

```rust
fn main() {
    // Comentário de uma linha
    /*
    Comentário
    Com mais de
    uma linha
    */
}
```

# ⚙️ [Rust] Variáveis, tipos e operadores de dados
No Rust, o uso de `"{}"` é peculiar porque ele faz parte do sistema de formatação de strings baseado em macros (`println!`, `format!`, `write!` etc.), e não de simples concatenação de strings como em algumas outras linguagens. A questão é, por que Rust usa `{}`? 

1. Pela segurança em tempo de compilação. Em Rust, quando você escreve:

```rust
println!("Olá, {}!", nome);
```

O compilador verifica se o número e tipo dos valores passados correspondem aos placeholders (`{}`). Se faltar argumento ou o tipo não implementar `Display` ou `Debug`, o código nem compila.

2. Baseado no **trait system**, que é um dos pilares da linguagem, é o mecanismo que permite definir comportamentos compartilhados e aplicá-los a diferentes tipos, funcionando de forma parecida com interfaces em outras linguagens, mas com mais poder e segurança.
   * `{}` é ligado ao **trait** `std::fmt::Display` → define como um tipo é convertido para string de forma "amigável".
   * `{:?}` usa `std::fmt::Debug` → formato para depuração (mais verboso, útil para debug structs).
   * Isso significa que a formatação é extensível — você pode criar seu próprio tipo e implementar `Display` para decidir como ele aparece com `{}`.

4. Separação de responsabilidade, diferente do `+` para concatenar strings, `{}` mantém o template de texto separado dos dados, tornando o código mais limpo e menos sujeito a erros de concatenação ou conversão de tipo manual.

5. Flexibilidade de formatação: Dentro das chaves, dá para colocar modificadores:

```rust
println!("{:>8}", 42);   // alinhado à direita em 8 colunas
println!("{:08}", 42);   // zero padding: 00000042
println!("{:.2}", 3.1416); // 3.14
```
     
O `"{}"` do Rust é uma instrução ao compilador para pegar um valor que implementa `Display` e gerar sua representação textual, validando os argumentos em tempo de compilação — algo que evita erros que, em outras linguagens, só apareceriam em runtime.

Exemplo:

```rust
fn main() {
   let x = 5;
   println!("{}",x+5);
}
```

Variáveis input:

```rust
use std::io;

// Funções
fn x_input(texto: &str) -> String {
    println!("{}", texto);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().to_string()
}

fn x_int(inteiro: &str) -> i32 {
    inteiro.trim().parse().expect("Falha ao converter para inteiro")
}

fn x_float(decimal: &str) -> f64 {
    decimal.trim().parse().expect("Falha ao converter para decimal")
}

fn main() {
    // Qual é o seu nome?
    let name = x_input("Qual é seu nome?");

    // Saída
    println!("Olá, {}!", name);

    // Digite um número inteiro
    let num_str = x_input("Digite um número inteiro:");
    let num: i32 = x_int(&num_str);
    let add = num + 1;

    // Saída
    println!("{} + 1 = {}", num, add);

    // Digite um número decimal
    let num_str = x_input("Digite um número decimal:");
    let num: f64 = x_float(&num_str);
    let div = num / 2.0;

    // Saída
    println!("{} / 2 = {}", num, div);
}use std::io;

// Funções
fn x_input(texto: &str) -> String {
    println!("{}", texto);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().to_string()
}

fn x_int(inteiro: &str) -> i32 {
    inteiro.trim().parse().expect("Falha ao converter para inteiro")
}

fn x_float(decimal: &str) -> f64 {
    decimal.trim().parse().expect("Falha ao converter para decimal")
}

fn main() {
    // Qual é o seu nome?
    let name = x_input("Qual é seu nome?");

    // Saída
    println!("Olá, {}!", name);

    // Digite um número inteiro
    let num_str = x_input("Digite um número inteiro:");
    let num: i32 = x_int(&num_str);
    let add = num + 1;

    // Saída
    println!("{} + 1 = {}", num, add);

    // Digite um número decimal
    let num_str = x_input("Digite um número decimal:");
    let num: f64 = x_float(&num_str);
    let div = num / 2.0;

    // Saída
    println!("{} / 2 = {}", num, div);
}
```

# ⚙️ [Rust] Módulos

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let mut controle = 0;
    let mut numero = true;

    while numero {
        println!("O número da vez é: {}", controle);
        controle += 1;
        numero = true;
        thread::sleep(Duration::from_secs(1));
    }
}
```

# ⚙️ [Rust] Arrays

## [Rust] Matrizes

# ⚙️ [Rust] Estruturas de programação

## [Rust] Estruturas condicionais

```rust
use std::io;

fn x_input(texto: &str) -> String {
    println!("{}", texto);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
    input.trim().to_string()
}

fn x_int(inteiro: &str) -> i32 {
    inteiro.parse::<i32>().expect("Falha ao converter para inteiro")
}

fn main() {
    let idade = x_int(&x_input("Qual sua idade: "));

    let acompanhado = true;

    if idade >= 18 {
        println!("Pode entrar e divirta-se!!!");
    } else if idade >= 16 && acompanhado {
        println!("Entre com o seu responsável!!!");
    } else {
        println!("Vai pra casa dormir criança!!!");
    }
}
```

## [Rust] Laços de Repetição (Loops)

# ⚙️ [Rust] FP - Paradigma Funcional

<img height="77" align="right" src="https://github.com/user-attachments/assets/fd0d00da-5f74-4701-ab9b-59403866138d" />

```rust
use std::io;

fn x_input(texto: &str) -> String {
    println!("{}", texto);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
    input.trim().to_string()
}

fn x_int(inteiro: &str) -> i32 {
    inteiro.parse::<i32>().expect("Falha ao converter para inteiro")
}

// Funções

fn soma(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let n1 = x_int(&x_input("Digite o primeiro número: "));
    let n2 = x_int(&x_input("Digite o segundo número: "));

    let resultado = soma(n1, n2);

    println!("O resultado da soma: {}", resultado);
}
```

# ⚙️ [Rust] OOP - Paradigma Orientado a Objetos
Rust é frequentemente descrito como uma linguagem **multiparadigma**, mas ela deliberadamente **não é OOP no sentido clássico** (como Java ou C#). Não existem classes, não existe herança de implementação, e isso é uma decisão de design, não uma limitação.

O que Rust NÃO tem

- **Classes** → em vez disso: `struct` (dados) + `impl` (comportamento) separados
- **Herança** → em vez disso: composição + traits
- **Polimorfismo por herança** → em vez disso: traits + generics ou trait objects

1. Encapsulamento: `struct` + `impl`

```rust
struct Usuario {
    nome: String,
    idade: u32,
}

impl Usuario {
    // "construtor" (convenção, não é especial na linguagem)
    fn novo(nome: &str, idade: u32) -> Self {
        Usuario { nome: nome.to_string(), idade }
    }

    // método (recebe &self)
    fn saudacao(&self) -> String {
        format!("Olá, {}", self.nome)
    }

    // método mutável
    fn fazer_aniversario(&mut self) {
        self.idade += 1;
    }
}
```

Encapsulamento é por **módulo**, não por classe: `pub` controla visibilidade no nível de módulo/crate, não existe `private`/`protected` por struct.

```rust
mod usuario {
    pub struct Usuario {
        pub nome: String,
        idade: u32, // privado — só acessível dentro do módulo
    }
}
```

2. Sem herança → Composição

Em vez de `class Gerente extends Funcionario`, você compõe:

```rust
struct Funcionario {
    nome: String,
    salario: f64,
}

struct Gerente {
    funcionario: Funcionario, // composição, não herança
    equipe: Vec<String>,
}

impl Gerente {
    fn nome(&self) -> &str {
        &self.funcionario.nome // delegação explícita
    }
}
```

Não há açúcar sintático automático para "herdar" métodos — você delega manualmente ou usa traits (abaixo). Isso é intencional: evita os problemas clássicos de hierarquias profundas de herança (fragilidade, diamond problem, etc).

3. Polimorfismo: Traits (equivalente a interfaces)

Traits são o mecanismo central de polimorfismo — similar a interfaces em Go/Java, mas mais poderosos (podem ter implementações default, tipos associados, etc):

```rust
trait Forma {
    fn area(&self) -> f64;

    // método default — como default methods em interfaces Java
    fn descricao(&self) -> String {
        format!("Área: {:.2}", self.area())
    }
}

struct Circulo { raio: f64 }
struct Retangulo { largura: f64, altura: f64 }

impl Forma for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }
}

impl Forma for Retangulo {
    fn area(&self) -> f64 {
        self.largura * self.altura
    }
}
```

Polimorfismo estático (generics) — zero-cost, resolvido em compile-time

```rust
fn imprimir_area<T: Forma>(forma: &T) {
    println!("{}", forma.descricao());
}
```

Polimorfismo dinâmico (`dyn Trait`) — como interface em Go/Java, com vtable em runtime

```rust
fn imprimir_area_dyn(forma: &dyn Forma) {
    println!("{}", forma.descricao());
}

let formas: Vec<Box<dyn Forma>> = vec![
    Box::new(Circulo { raio: 2.0 }),
    Box::new(Retangulo { largura: 3.0, altura: 4.0 }),
];

for f in &formas {
    println!("{}", f.descricao());
}
```

4. Comparando com o que você já usa

| Conceito OOP | Java/Spring | Go | Rust |
|---|---|---|---|
| Dados + comportamento | `class` | `struct` + funções | `struct` + `impl` |
| Interface | `interface` | `interface` (implícita) | `trait` |
| Herança de implementação | `extends` | ❌ não existe | ❌ não existe |
| Polimorfismo runtime | virtual dispatch | interface satisfeita implicitamente | `dyn Trait` (vtable) |
| Polimorfismo compile-time | ❌ (type erasure em generics) | ❌ (até genéricos chegarem, e mesmo assim limitado) | generics com monomorfização |
| Encapsulamento | `private`/`protected`/`public` na classe | maiúscula/minúscula no pacote | `pub` no módulo |

Se você vem de Spring Boot, o paralelo mental mais próximo é: **trait ≈ interface Java**, mas sem a possibilidade de "extends" — tudo é composição + implementação de traits.

5. Padrões que substituem heranças comuns em Java

- **Strategy pattern** → generics ou `dyn Trait` diretamente, sem precisar do boilerplate de interface + classes concretas
- **Template method** → métodos default em traits
- **Builder pattern** → muito comum em Rust justamente por não haver construtores com múltiplos overloads

```rust
// Builder — comum no lugar de construtores telescópicos
struct RequisicaoBuilder {
    url: String,
    timeout: Option<u32>,
}

impl RequisicaoBuilder {
    fn novo(url: &str) -> Self {
        Self { url: url.to_string(), timeout: None }
    }

    fn timeout(mut self, segundos: u32) -> Self {
        self.timeout = Some(segundos);
        self
    }
}
```

# ⚙️ [Rust] Programação assíncrona
Rust trata assincronismo de forma diferente de Go (goroutines) ou até de Python/JS (event loop embutido): a linguagem fornece a **sintaxe** (`async`/`await`), mas não inclui um runtime — você escolhe um externo.

1. `async`/`await` básico

```rust
async fn buscar_dados() -> String {
    // código assíncrono
    "dados".to_string()
}

async fn processar() {
    let resultado = buscar_dados().await;
    println!("{}", resultado);
}
```

Uma função `async fn` não executa nada sozinha — ela retorna um **`Future`**, que é "preguiçoso" (lazy). Nada acontece até que algo o execute (`.await` ou um runtime).

2. Runtimes: Tokio é o padrão de fato

Sem runtime, `Future`s não rodam. O mais usado de longe é o **Tokio**:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let resultado = buscar_dados().await;
    println!("{}", resultado);
}
```

O macro `#[tokio::main]` é açúcar sintático que transforma `main` numa função síncrona que cria o runtime e roda o `Future` até completar.

Alternativas: **async-std** (API parecida com a std), **smol** (minimalista). Tokio domina o ecossistema (web frameworks, bancos, etc quase todos assumem Tokio).

3. Concorrência: rodando tarefas em paralelo

`tokio::spawn` — tarefas independentes (parecido com goroutine)

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // roda concorrentemente
        buscar_dados().await
    });

    let resultado = handle.await.unwrap();
    println!("{}", resultado);
}
```

`join!` — esperar várias futures ao mesmo tempo

```rust
use tokio::join;

async fn main() {
    let (a, b) = join!(buscar_dados(), buscar_outros_dados());
}
```

`select!` — corrida entre futures, pega a primeira que terminar

```rust
use tokio::select;

async fn exemplo() {
    select! {
        resultado = buscar_dados() => println!("dados: {}", resultado),
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            println!("timeout!");
        }
    }
}
```

4. Diferença conceitual: Rust vs Go

Como você trabalha com Go, vale destacar o contraste:

| | Go | Rust |
|---|---|---|
| Modelo | Goroutines + canais, agendadas pelo runtime Go embutido | `Future`s baseados em polling, precisam de executor externo (Tokio) |
| Custo | Leve, mas com stack própria por goroutine | Zero-cost abstraction — vira uma máquina de estados em tempo de compilação |
| Cancelamento | Cooperativo via `context.Context` | `Future` dropado = cancelado automaticamente |
| Erro comum | Vazamento de goroutine | "Future não é `Send`", problemas de lifetime em código async |

Um `Future` em Rust é essencially um enum/state machine gerado pelo compilador — não há overhead de heap allocation a menos que você use `Box::pin` explicitamente (ex: para recursão em funções async).

5. Traits assíncronos em structs

Isso costumava ser dor de cabeça (precisava do crate `async-trait`), mas desde Rust 1.75 há suporte nativo a `async fn` em traits (com limitações — não é dyn-compatible por padrão):

```rust
trait Buscador {
    async fn buscar(&self, id: u32) -> Option<String>;
}
```

6. Erros em código async

Combina normalmente com `Result` e `?`:

```rust
async fn buscar_e_processar() -> anyhow::Result<String> {
    let dados = reqwest::get("https://api.exemplo.com")
        .await?
        .text()
        .await?;
    Ok(dados)
}
```

# ⚙️ [Rust] Tratamento de exceções
Tratamento de Erros em Rust não tem exceções no sentido tradicional (como `try/catch` em Java, Python, etc). Em vez disso, usa um sistema baseado em **tipos de retorno** para tratar erros, dividido em duas categorias principais:

1. Erros Recuperáveis: `Result<T, E>`

Para erros que você espera que possam acontecer e quer tratar (arquivo não encontrado, parse inválido, etc):

```rust
use std::fs::File;
use std::io::{self, Read};

fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?; // operador ? propaga o erro
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}

fn main() {
    match ler_arquivo("dados.txt") {
        Ok(conteudo) => println!("Conteúdo: {}", conteudo),
        Err(e) => eprintln!("Erro ao ler arquivo: {}", e),
    }
}
```

O operador `?` é o coração do tratamento de erros idiomático em Rust. Ele propaga o erro automaticamente para o chamador, encurtando muito o código comparado a `match` explícito em cada chamada.

```rust
fn processar() -> Result<i32, String> {
    let valor = "42".parse::<i32>().map_err(|e| e.to_string())?;
    Ok(valor * 2)
}
```

2. Erros Irrecuperáveis: `panic!` para erros que indicam um bug ou estado inválido do programa, onde continuar não faz sentido:

```rust
fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divisão por zero!");
    }
    a / b
}
```

Métodos como `.unwrap()` e `.expect("mensagem")` também causam panic se o `Result`/`Option` for `Err`/`None`:

```rust
let numero: i32 = "abc".parse().expect("deveria ser um número válido");
// panic com a mensagem se falhar
```

**Regra geral**: use `panic!` só quando o erro representa um bug (violação de invariante), não para erros esperados do mundo real (arquivo ausente, entrada inválida do usuário, etc — esses merecem `Result`).

3. Tipos de erro customizados para projetos maiores, é comum criar um `enum` de erro próprio:

```rust
use std::fmt;

#[derive(Debug)]
enum MeuErro {
    ArquivoNaoEncontrado,
    FormatoInvalido(String),
}

impl fmt::Display for MeuErro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeuErro::ArquivoNaoEncontrado => write!(f, "arquivo não encontrado"),
            MeuErro::FormatoInvalido(msg) => write!(f, "formato inválido: {}", msg),
        }
    }
}

impl std::error::Error for MeuErro {}
```

Na prática, para isso costuma-se usar crates como:

- **`thiserror`** — para definir tipos de erro customizados com boilerplate mínimo (bom para bibliotecas)
- **`anyhow`** — para propagar erros de forma genérica com contexto (bom para aplicações)

```rust
// Com anyhow
use anyhow::{Context, Result};

fn processar_config() -> Result<()> {
    let conteudo = std::fs::read_to_string("config.toml")
        .context("falha ao ler arquivo de configuração")?;
    Ok(())
}
```

4. `Option<T>` para ausência de valor não é exatamente "erro", mas segue o mesmo padrão para representar "pode não haver valor":

```rust
fn buscar_usuario(id: u32) -> Option<String> {
    if id == 1 {
        Some("Isaac".to_string())
    } else {
        None
    }
}
```

# ⚙️ [Rust] Tokio
**Tokio** é muito mais do que uma simples biblioteca na ecologia da linguagem Rust, ela é o alicerce sobre o qual a programação assíncrona prática e de alto desempenho é construída no mundo Rust. Em sua essência, o Tokio é um **runtime assíncrono** que fornece os blocos fundamentais necessários para escrever aplicações de rede que são simultaneamente confiáveis e extremamente eficientes em termos de recursos.

A revolução que o Tokio traz está em como ele permite que o Rust, uma linguagem de programação de sistemas inerentemente sínnca e sem um runtime de coleta de lixo, execute operações de E/S que são tradicionalmente bloqueantes — como aguardar a chegada de um pacote de rede, ler um arquivo do disco ou estabelecer uma conexão de banco de dados — de forma não bloqueante. Ele consegue isso através de um modelo de **multitarefa cooperativo** que é acionado pela sintaxe `async`/`await` do Rust. Quando uma função é marcada como `async`, ela não executa um código imediatamente; em vez disso, ela retorna um `Future`, que é uma promessa de um valor que pode não estar disponível ainda. O runtime do Tokio é o motor que pega esses milhares ou mesmo milhões de `Futures` e os executa em um pequeno número de threads do sistema operacional, trocando entre eles de forma inteligente sempre que um deles é bloqueado por E/S.

A arquitetura do Tokio é construída em torno de alguns componentes centrais que trabalham em harmonia. O **Agendador de Trabalho** é o cérebro do runtime, responsável por decidir qual `Future` deve ser executado a seguir em cada uma das threads de trabalho. Ele é altamente otimizado para reduzir ao máximo a latência e o custo de troca de contexto entre tarefas. O **Sistema de E/S** do Tokio é a sua interface com o mecanismo de E/S do sistema operacional, usando as melhores abstrações disponíveis em cada plataforma — como `epoll` no Linux, `kqueue` no macOS e BSD, e `IOCP` no Windows — para ser notificado quando uma operação de E/S estiver concluída, permitindo que ele retome a `Future` correta no momento exato. O **Timer** é um componente crucial para agendar eventos futuros, como timeouts de rede ou execução de tarefas periódicas, sendo essencial para a construção de aplicações responsivas.

O ecossistema Tokio vai muito além do runtime núcleo, fornecendo uma suíte abrangente de ferramentas que cobrem as necessidades mais comuns no desenvolvimento de servidores. Ele oferece uma implementação robusta de **TCP, UDP e Unix Domain Sockets**, abstrações de alto nível para **clientes e servidores de rede**, utilitários para **processamento simultâneo de canals**, e um **sistema de arquivos assíncrono** que permite operações de E/S de disco não bloqueantes. Esta abordagem de "baterias incluídas" é o que torna o Tokio tão produtivo; em vez de ter que escolher e integrar bibliotecas diferentes para cada protocolo de rede, o desenvolvedor encontra uma stack coesa e interoperável que já foi amplamente testada em produção.

A influência do Tokio é tão profunda que ele se tornou o runtime assíncrono *de facto* para a maioria dos projetos Rust. Bibliotecas de cliente para bancos de dados, clientes HTTP de alto nível como o `reqwest`, e frameworks web modernos como o `Axum` e o `warp` são todos construídos em cima do Tokio, pressupondo sua presença. Esta padronização tácita cria uma enorme sinergia no ecossistema, permitindo que diferentes bibliotecas funcionem perfeitamente juntas, pois todas compartilham o mesmo runtime subjacente e o mesmo modelo de execução.

No final, Tokio é a materialização do modelo de concorrência de Rust em sua forma mais prática e poderosa. Ele permite que os desenvolvedores escrevam servidores que podem lidar com dezenas ou mesmo centenas de milhares de conexões concorrentes em uma única máquina, com um consumo de memória notavelmente baixo e sem a complexidade e os riscos de segurança associados ao *threading* tradicional do sistema operacional. Ele é a peça que falta para que Rust realize sua promessa de permitir um software de sistema que é não apenas seguro contra erros de memória, mas também incrivelmente eficiente na utilização de recursos para cargas de trabalho de rede e E/S intensivas.

Três meses atrás, tomei uma decisão que poderia ter me demitido. Nosso microsserviço Rust estava sangrando dinheiro. As contas da AWS sobem todos os meses. A latência está aumentando. A equipe estava adicionando mais instâncias, mais memória, mais tudo. Problema de dimensionamento clássico, certo? Errado!

Passei um fim de semana arrancando todo o nosso tempo de execução assíncrono. Excluído Tokio. Futuros substituídos por fios simples. Voltei a bloquear E/S como um dinossauro escrevendo código C em 1995.

Na segunda-feira de manhã, coloquei-o em produção.

O canal do Slack ficou em silêncio. Então as métricas começaram a chegar.

O uso de memória caiu 42%. Os tempos de resposta melhoraram em todos os percentis. A latência do P99 caiu de 340ms para 180ms. As projeções de custo mostraram que economizaríamos mais de US$ 127.000 anualmente.

Meu gerente me fez uma pergunta: "Por que não fizemos isso antes?" Boa pergunta.

A mentira assíncrona em que todos acreditávamos, Deixe-me levá-lo de volta a quando aprendi Rust pela primeira vez. Cada tutorial, cada postagem de blog, cada palestra de conferência tinha a mesma mensagem: assíncrono é o futuro. Tokio é essencial. Se você não está escrevendo código assíncrono, está preso no passado.

# ⚙️ [Rust] WebAssembly
<a href="https://webassembly.org/"><img src="https://cdn.worldvectorlogo.com/logos/webassembly-1.svg" height="77" align="right"></a>

O **WebAssembly**, abreviado como **Wasm**, é um formato de instrução binária para uma máquina virtual baseada em pilha. O Wasm foi projetado como um destino de compilação portátil para linguagens de programação, permitindo a implantação na Web para aplicativos de cliente e servidor. 

WebAssembly é um formato de código binário portátil cujo objetivo é tornar possíveis aplicações de altíssima performance em páginas Web, ao mesmo tempo em que adiciona pluralidade, pois viabiliza que programadores de diversas linguagens criem ou portabilizem aplicações existentes de plataformas nativas para os navegadores. Tudo isso tendo um modelo de segurança do WASM projetado para ser completamente isolado em "caixa de areia" (sandbox), com permissões restritas e controladas pela pessoa desenvolvedora. Tendo conhecimento aprofundado, é possível utilizar WebAssembly sem depender de linguagem, ferramenta ou recurso específico além dele mesmo para desenvolver aplicações, seja para a Web ou até mesmo no back-end e outros destinos de uso.

> O Wasm é basicamente um bytecode, é um código intermediário resultante da compilação de uma linguagem e que será interpretado por uma máquina virtual (VM) para então ser transformado em linguagem de máquina e logo será interpretado pelo seu computador, contextualizando ele é uma linguagem de baixo-nível, mais próximo da linguagem de máquina. Assim, como a linguagem de montagem ou Assembly.

A intenção do Web Assembly é ser interpretado pela VM para depois poder ser interpretado pelos navegadores permitindo que qualquer linguagem gere esse bytecode possa ser utilizada para o seu desenvolvimento Web.

O objetivo do Web Assembly é de se tornar uma tecnologia universal para desenvolvimento de aplicações Web melhorando consideravelmente a performance e disponibilizando os recursos de aplicações nativa para o desenvolvimento Web, ou seja, isso abre um novo horizonte para toda a internet, onde os desenvolvedores terão um leque de possibilidades na criação de aplicações e os usuários teram a mesma experiência de aplicações nativas no seu computador.

Imagina a hipótese de rodar qualquer tipo de aplicação no seu navegador, aplicativos como: jogos, ferramentas como Adobe Photoshop CC, Autodesk Autocad e até mesmo aplicativos móveis. E isso tudo sem ter que se preocupar com memória interna, pois não precisará instalar nenhuma aplicação. Estamos falando em um mundo compatível com todos os sistemas operacionais e dispositivos, tudo dentro do navegador. 

Atualmente, as linguagens de programação que suportam/compilam o módulo Web Assembly são: 

- C/C++, Rust (muito popular),
- AssemblyScript (uma sintaxe TypeScript),
- C#, F#, Go, Kotlin, Swift, D, Pascal, Zig, Grain, Python (experimental).

Essas linguagens são as que suportam e possuem uma estrutura/arquitetura ideal para desenvolvimento em baixo-nível e com elas você terá um maior controle da sua aplicação podendo arquitetar do jeito que você achar necessário em pontos cruciais, como por exemplo: O gerenciamento de memória é um dos principais pontos fracos de linguagens dinâmicas como JavaScript. Ou seja, podemos usar o WebAssembly compilado pelo código JavaScript, como uma aplicação CLI e pelo Node.js (com acesso aos recursos do sistema).

A máquina de pilha Wasm foi projetada para ser codificada em um formato binário com eficiência de tamanho e tempo de carregamento. O WebAssembly visa executar em velocidade nativa, aproveitando os recursos de hardware comuns disponíveis em uma ampla variedade de plataformas.

O WebAssembly descreve um ambiente de execução em área restrita com proteção de memória que pode até ser implementado dentro de máquinas virtuais JavaScript existentes. Quando incorporado na Web, o WebAssembly aplicará as políticas de segurança de mesma origem e permissões do navegador.

O WebAssembly foi projetado para ser impresso em um formato textual para depuração, teste, experimentação, otimização, aprendizado, ensino e escrita de programas à mão. O formato textual será usado ao visualizar a fonte dos módulos Wasm na web. E, também, para manter a natureza sem versão, com recursos testados e compatível com versões anteriores da web. Os módulos WebAssembly poderão entrar e sair do contexto JavaScript e acessar a funcionalidade do navegador por meio das mesmas APIs da Web acessíveis a partir do JavaScript. O WebAssembly também oferece suporte a incorporações não-web.

O que é montagem de teia (WASM)? Por que atrai tanta atenção? O diagrama mostra como podemos executar código C/C++/Rust nativo dentro de um navegador da Web com WASM.

<img width="1456" height="1362" alt="image" src="https://github.com/user-attachments/assets/62ded1b2-462a-49eb-a583-c80ea2360fb7" />

Tradicionalmente, só podemos trabalhar com Javascript no navegador da web, e o desempenho não pode ser comparado com código nativo como C/C++ porque é interpretado.

No entanto, com o WASM, podemos reutilizar bibliotecas de código nativas existentes desenvolvidas em C/C++/Rust, etc. para serem executadas no navegador da web. Esses aplicativos da Web têm desempenho quase nativo.

Por exemplo, podemos executar a biblioteca de codificação/decodificação de vídeo (escrita em C++) no navegador da web.

Isso abre muitas possibilidades para computação em nuvem e computação de borda. Podemos executar aplicativos sem servidor com menos recursos e tempo de inicialização instantâneo.

# ⚙️ Zig
**Zig** é uma linguagem de programação de baixo nível, moderna e deliberadamente minimalista, criada para ocupar o espaço entre C e linguagens de sistemas mais “opiniosas”, como Rust, mas sem tentar substituir completamente nenhuma delas. Ela foi projetada com um foco muito claro em previsibilidade, controle explícito e simplicidade sem magia, partindo da ideia de que o programador deve entender exatamente o que o código faz, quando faz e quanto custa em termos de recursos.

A filosofia central do Zig é que não deve existir comportamento implícito escondido do desenvolvedor. Diferente de muitas linguagens modernas, Zig não tem garbage collector, não tem exceções no sentido tradicional, não tem runtime pesado e não depende de um sistema operacional específico para funcionar. O gerenciamento de memória é explícito e sempre visível no código, mas feito de forma estruturada, através de allocators passados como dependência, o que permite escrever código altamente previsível e testável, algo muito valorizado em sistemas críticos, embarcados e de alta performance.

Um dos pontos mais marcantes do Zig é o tratamento de erros. Em vez de exceções ou códigos de erro soltos, Zig usa valores de erro como parte do sistema de tipos. Uma função pode retornar um valor ou um erro, e o compilador força o programador a lidar com isso explicitamente. Isso evita uma enorme classe de bugs silenciosos e torna o fluxo de erro claro no próprio código, sem necessidade de pilhas de exceção ou mecanismos ocultos. O erro deixa de ser algo “especial” e passa a ser parte do contrato da função.

Zig também se destaca pelo conceito de compile-time extremamente poderoso. O código Zig pode ser executado em tempo de compilação, permitindo gerar código, validar invariantes, configurar estruturas e adaptar comportamentos sem recorrer a macros complexas ou ferramentas externas. Isso faz com que Zig seja incrivelmente expressiva sem perder legibilidade. O compilador se torna quase um interpretador controlado pelo programador, mas de forma explícita e segura.

Outro aspecto fundamental é a relação do Zig com C. Zig não tenta “esconder” C nem substituí-lo agressivamente. Pelo contrário, ele foi projetado para interoperar com C de forma quase perfeita. Você pode importar headers C diretamente, chamar funções C sem wrappers artificiais e até usar Zig como um compilador C alternativo, aproveitando o sistema de build e o cross-compilation extremamente robusto da linguagem. Isso faz do Zig uma ferramenta muito atraente para modernizar bases de código existentes ou escrever bibliotecas que precisam conversar com o ecossistema C sem fricção.

Falando em build system, Zig integra o sistema de build à própria linguagem, eliminando a dependência de ferramentas externas complexas como Make, CMake ou autotools. O build é descrito em Zig, com lógica real, tipos e validações, o que torna builds mais previsíveis, portáveis e fáceis de manter. Além disso, o suporte a cross-compilation é um dos pontos mais fortes da linguagem: é possível compilar para múltiplas arquiteturas e sistemas operacionais a partir de uma única máquina, sem cadeias de ferramentas externas, algo que costuma ser doloroso em C e C++.

Em termos de segurança, Zig não tenta ser “memory safe por padrão” como Rust, mas fornece ferramentas explícitas para escrever código seguro quando o desenvolvedor assim deseja. Ele permite tanto código extremamente próximo do metal quanto abstrações seguras, desde que essas abstrações não escondam custo nem comportamento. Isso faz com que Zig seja frequentemente descrita como uma linguagem “honesta”: ela não promete te salvar de todos os erros, mas também não te engana.

No cenário real, Zig vem sendo adotada para desenvolvimento de sistemas embarcados, motores de jogos, ferramentas de sistema, compiladores, runtimes, bibliotecas de alto desempenho e infraestrutura de baixo nível. Ela ainda não tem o mesmo ecossistema de linguagens mais antigas ou mainstream, mas cresce de forma consistente justamente porque resolve problemas reais sem adicionar complexidade artificial.

Em essência, Zig é uma linguagem para quem quer controle, clareza e previsibilidade, sem abrir mão de expressividade moderna. Ela não tenta ser tudo para todos, mas é extremamente eficaz naquilo a que se propõe: permitir que o programador escreva software de baixo nível com menos dor, menos mágica e mais entendimento real do que está acontecendo.

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
