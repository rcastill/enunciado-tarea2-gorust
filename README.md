# GoRust: Tarea 2

La tarea consiste en implementar un diccionario rudimentario en el lenguaje de programación Rust. Debe estar basado en un árbol binario, donde cada nodo será un par `(key, value)` y deberán agregarse ordenados dependiendo del valor de la llave.

Un ejemplo conceptual (trabajando sólo con tuplas):

```
# árbol vacío
()

# árbol con el par (42, 10) en su raíz
((42, 10), nil, nil)

# si ahora ingresamos el par (11, 32), se agregará como hijo izquierdo de la raíz, ya que la llave es menor que la del padre: 11 < 42
((42, 10), ((11, 32), nil, nil), nil, nil)

# si agregamos el par (14, 9), se agregará como hijo derecho del segundo nodo, ya que: 14 < 42, pero 14 > 11
((42, 10), ((11, 32), nil, ((14, 9), nil, nil)), nil, nil)
```

Este debe ser el comportamiento de su algoritmo, de esta manera acortará el tiempo de búsqueda de un nodo (búsqueda binaria). El proceso se optimiza más aún si se balancea el árbol, pero no es un objetivo de la tarea.

**Nota:** no necesariamente debe ser representado con tuplas en su implementación. Utilice la estructura de datos que más le acomode mientras genere un árbol binario.

## Implementación

En la sección anterior se indica la teoría del objetivo. En código esto se traducirá a 3 archivos, de los cuales 1 tendrá que ser manipulado por usted:

```
src/
  factory.rs (1)
  main.rs (2)
  tree_dict.rs (3)
```

1. El archivo `factory.rs` contiene el trait `Map<K, V>` que define los métodos `insert`, `get` y `has_key`.

   `insert`: este método ingresará el par `(key, value)` en el diccionario.

   `get`: este método obtendrá el valor dada una llave.

   `has_key`: indica si es que el diccionario tiene una llave dada (implementado por defecto ya que se puede obtener la respuesta a partir de `get`)

2. El archivo `main.rs` contiene los tests que debe pasar para aprobar la tarea.

3. El archivo `tree_dict.rs` contiene porciones de código para rellenar.

   En este archivo además tendrá que crear el tipo `TreeDict<K, V>` y lograr que su implementación calce con los requerimientos de la estructura del programa.

## Restricciones

- Los archivos (1) y (2) no deben ser modificados
- El archivo (3) puede ser modificado a discreción, considerando siempre que el aspecto más importante es implementar el trait `Map`.

## Instrucciones

- Deberá instalar Rust en su computador. Siga las instrucciones en la [página oficial](https://www.rust-lang.org/en-US/install.html)
- Para ejecutar los tests corra `cargo test` desde el directorio raíz
- Para realizar la entrega, realice un `fork` del repositorio y agregue a `rcastill` como contribuidor de su repositorio.
- Ante cualquier consulta mande tantos correos como sea necesario a [rodolfocastillomateluna@gmail.com](mailto:rodolfocastillomateluna@gmail.com)
- **Fecha límite:** 15 de Diciembre de 2017 hasta las 23:55.