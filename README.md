<h1 align="center">Gerador de frase seed para carteiras bitcoin</h1>

<h3 align="center"> Aplicação desenvolvida em Rust </h3>

</p>
<p align="center">
  <a href="./LICENSE">
    <img src="https://img.shields.io/github/license/GabrielPCO/btc-seed-generator?color=blue">
  </a>
  <img src="https://img.shields.io/badge/contributions-welcome-orange">
  <a href="https://github.com/GabrielPCO/btc-seed-generator/stargazers">
    <img src="https://img.shields.io/github/stars/GabrielPCO/btc-seed-generator">
  </a>
  <a href="https://github.com/GabrielPCO/btc-seed-generator/network">
    <img src="https://img.shields.io/github/forks/GabrielPCO/btc-seed-generator">
  </a>
</p>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

## Conteúdo
- [Aplicação](#aplica%C3%A7%C3%A3o)
- [Requisitos](#requisitos)
- [Instalação](#instala%C3%A7%C3%A3o)
- [Guia](#guia)

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

## Aplicação
Aplicação de CLI em Rust para a geração de uma frase seed aleatória para carteiras bitcoin 

> [!CAUTION]
> Apenas para uso educacional! Os números randômicos neste projeto **NÃO** são gerados com segurança adequada, por isso não deve ser utilizada para carteiras reais de bitcoin

## Requisitos
- Windows ou MacOS (Testado em MacOS com cpu Intel)
- Ambiente de programação Rust instalado na máquina

## Instalação
- 1: Clone o respositório para a sua máquina com o comando git clone
```

git clone https://github.com/GabrielPCO/btc-seed-generator.git

```

- 2: Acesse a pasta clonada
```

cd btc-seed-generator

```

- 3: Realiza a build da aplicação
```

cargo build

```

- 4: Acesse a pasta /src
```

cd src

```

- 5: Execute a aplicação
```

cargo run

```

## Guia
```
1. Após executar a aplicação, aguarde sua inicialização
2. Digite [1] para a geração de frase seed de 24 palavras ou [2] para 12 palavras. Pressione a tecla [ENTER]
3. Mova o mouse para gerar aleatoriedade na criação da frase seed
4. Processo concluído, sua frase seed será impressa no terminal

```
