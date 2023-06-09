# ¡Hola a todos! 😃 Bienvenidos a guess-number-rust 👋

Este es un pequeño proyecto que hice para aprender Rust. ¡Es un juego sencillo, pero me divertí mucho haciéndolo! 

## ¿De qué trata guess-number-rust? 🤔

Bueno, como puedes adivinar por el nombre, es un juego en Rust donde tienes que adivinar un número. El programa genera un número aleatorio entre 1 y 100 y tu trabajo es adivinarlo. Si tu predicción es demasiado alta, el programa te dirá que bajes un poco. Si es demasiado baja, te dirá que subas. ¡Y si aciertas, ganas!

## ¿Cómo puedo jugar? 🎮

Primero, necesitas tener Rust instalado en tu sistema. Si no lo tienes, puedes seguir [este enlace](https://www.rust-lang.org/tools/install) para instalarlo.

Una vez que tengas Rust, puedes clonar este repositorio y ejecutar el juego con `cargo run`.

## ¿Como funciona?
Al inicio, se genera un número secreto aleatorio entre 1 y 100 utilizando la librería `rand`. Luego, el programa entra en un bucle infinito, pidiendo al usuario que introduzca un número como su predicción.

El programa lee la entrada del usuario, intentando convertirla en un número entero de 32 bits (`u32`). Si la conversión falla (es decir, el usuario introduce algo que no es un número), el programa imprimirá un mensaje de error y volverá al inicio del bucle, pidiendo otra predicción.

Si la conversión tiene éxito, el programa compara la predicción con el número secreto. Si la predicción es más baja que el número secreto, imprime "¡Muy bajo!". Si es más alta, imprime "¡Muy alto!". Si la predicción es correcta, imprime "¡Acertaste!", y luego rompe el bucle, terminando el programa.


```bash
git clone https://github.com/tu_nombre_de_usuario/guess-number-rust.git
cd guess-number-rust
cargo run
