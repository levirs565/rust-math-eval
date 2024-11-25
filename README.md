# Rust Math Eval

My tiny toy project to parse math expression, convert to expression tree and evaluate the tree.

```
Masukkan ekspresi : 1 + 2 * 3 + (2 + 3) * 4 + (78 / 2 ^ (23 * 3)) * (20 + 7) 
+
|---+
|   |---+
|   |   |---1
|   |   |---*
|   |       |---2
|   |       |---3
|   |
|   |
|   |---*
|       |---+
|       |   |---2
|       |   |---3
|       |
|       |---4
|
|
|---*
    |---/
    |   |---78
    |   |---^
    |       |---2
    |       |---*
    |           |---23
    |           |---3
    |
    |
    |
    |---+
        |---20
        |---7



Evaluated to: 27
```

## Syntax

```
TermBaseItem = Number | (Brace Open, Expression, Brace Close)
Exponent = TermBaseItem | (TermBaseItem, ^, TermBaseItem)
TermItem = TermBaseItem | Exponent
Term = TermItem | (TermItem, * | /, Term) | 
Expression = Term | (Term, + | -, Term)
```