# tp1-concurrentes-1-fiuba
Repositorio para el Trabajo Practico 1 en Técnicas de Programación Concurrentes 1

Para correr el programa:

```
cargo run
```

Para correr los tests:

```
cargo test
```

Para correr el linter:

```
cargo clippy
```

Los tests fueron creados siguiendo la siguiente estructura:

```
// foo.rs
pub mod foo {
    pub fn foo(a: i32, b: i32) -> i32 {
        a + b
    }    
}

#[cfg(test)]
#[path = "./foo_test.rs"]
mod foo_test;
```

```
// foo_test.rs
use super::foo;

#[test]
fn it_adds() {
    assert_eq!(3, foo::foo(1, 2));
}
```

De esta manera se pueden separar los tests de la funcionalidad en archivos separados.