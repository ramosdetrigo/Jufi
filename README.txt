Júlia Andrade Ramos - 558279

Bom dia, tarde ou noite Gilvan! :)

Vou organizar todos os trabalhos nesse projetinho em Rust.
Tentei comentar bem o código para explicar o que estou fazendo.

Esta atividade corresponde ao portfólio B1 - Interseção de segmentos.
O programa já sabe identificar a colisão entre dois segmentos AB e CD,
e dizer qual o ponto de colisão entre os dois segmentos.

Ele tecnicamente também já sabe identificar retas degeneradas e paralelas,
mas isso ainda não está integrado com o detector de colisão.

No código atual, existem duas retas:
- reta1, definida numa localização aleatória
- reta_mouse, que segue a posição do mouse.

A reta1 é a reta laranja e a reta_mouse é a verde.
Se as retas se interceptam, a reta_mouse fica vermelha
e é desenhado o ponto de interseção em roxo.

Comandos:
- barra de espaço: randomiza a posição da reta1.
- mover o mouse: faz o ponto p2 da reta_mouse seguir o mouse.
- left click: faz o ponto p1 da reta_mouse ir pra posição atual do mouse.

Utilizei a biblioteca "macroquad" exclusivamente para
desenhar as coisas na tela.

O projeto vem com o binário já compilado para Linux x86-64,
mas para compilar e rodar o código, basta ter
a ferramenta "cargo" instalada (i.e. ter Rust instalado no seu PC)
e rodar "cargo run" na raiz do projeto.

Segue abaixo a página de "get started" de Rust para referência
https://rust-lang.org/learn/get-started/

Sugestão: Instalar a extensão rust-analyzer para o VSCode, Vim, Neovim
ou qualquer seja o editor de texto que você usa.
