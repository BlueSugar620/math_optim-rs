# Sparse Table

## definition

Let
$(M, \cdot)$
be a band.

### construct

Given
$A = (A_i)_{i = 0}^{n - 1}$
,
struct sparse table.

It takes
$\langle O(n \log n), O(n \log n) \rangle$
complexity.

### fold

Given interval
$I$
,
return the value
$\prod_{i \in I} A_i$
.

It takes 
$O(1)$
time complexity.

### point-at get

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return
$A_i$
.

It takes
$O(1)$
time complexity.
