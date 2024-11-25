
## Syntax

```
TermBaseItem = Number | (Brace Open, Expression, Brace Close)
Exponent = TermBaseItem | (TermBaseItem, ^, TermBaseItem)
TermItem = TermBaseItem | Exponent
Term = TermItem | (TermItem, * | /, Term) | 
Expression = Term | (Term, + | -, Term)
```