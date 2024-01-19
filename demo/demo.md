An array is a numbered, fixed-size sequence of elements of a single [type](http://link_to_type_spec/), called the element type. The element type of an array can be any Rust [type](http://link_to_type_spec/). 

The size of an array must be specified by a constant expression that evaluates to a non-negative value of type `usize`.

An array's type is the combination of its size and element type. An array's type cannot change after declaration.

Array elements are stored contiguously in memory. The elements of an array are immutable unless the array is declared mutable.

## Array Type Specification

$$ 
\begin{align*}
array\text{\text{-}}type ::=& \texttt{[} any\text{-}type ; array\text{-}size \texttt{]} \texttt{=} array\text{-}initializer\\

any\text{-}type ::=& \texttt{[}type\texttt{]} \\

array\text{-}size ::= & \texttt{[}size\texttt{]} \\

array\text{-}initializer ::=& \\
    & \texttt{[} array\text{-}element\text{-}list \texttt{]} \\
    & \texttt{[} repeat\text{-}expression \texttt{]} \\

array\text{-}element\text{-}list ::=& \\
    & array\text{-}element \\
    & array\text{-}element\text{-}list \texttt{,} array\text{-}element \\

array\text{-}element ::=& expression \\

repeat\text{-}expression ::=& \texttt{[} expression \texttt{;} usize \texttt{]} \\
\end{align*}
$$

### Examples

```Rust
let mut _spec_a: [i32; 3] = [3; 3]; // repeat expression
let _spec_b: [String; 2] = [String::from("hi"), String::from("bye")];
let _spec_c: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
let _spec_d: [u64; 0] = [];
```