# Fenwick Tree on Monoid

## Definition

Let
$(M, \cdot)$
be a commutative monoid.

### construct

Given
$A = (A_i)_{i = 0}^{n - 1}$
which is the sequence on
$M$
,
struct Fenwick tree.

It takes
$\langle O(n), O(n) \rangle$
complexity.

### push

Given
$x \in M$
,
push
$x$
to
$A$
.

It takes
$O(\log n)$
complexity.

### point-at operate

Given
$i \in [0, n)$
and
$x \in M$
,
update
$A_i$
to
$A_i \cdot x$
.

It takes
$O(\log n)$
time complexity.

### prefix sum

Given
$r \in [0, n)$
,
return
$\prod_{i = 0}^{r - 1} A_i$
.

It takes
$O(\log n)$
time complexity.

# Fenwick Tree on Abelian

## Definition

Let
$(M, \cdot)$
be a abelian group.

### construct

Given
$A = (A_i)_{i = 0}^{n - 1}$
which is a sequence on
$M$
,
struct fenwick tree.

It takes
$\langle O(n), O(n) \rangle$
complexity.

### push

Given
$x \in M$
,
push
$x$
to
$A$
.

It takes
$O(\log n)$
complexity.

### point-at operate

Given
$i \in [0, n)$
and
$x \in M$
,
update
$A_i$
to
$A_i \cdot x$
.

It takes
$O(\log n)$
time complexity.

### point-at update

Given
$i \in [0, n)$
and
$x \in M$
,
update
$A_i$
to
$x$
.

It takes
$O(\log n)$
time complexity.

### prefix sum

Given
$r \in [0, n)$
,
return
$\prod_{i = 0}^{r - 1} A_i$
.

It takes
$O(\log n)$
time complexity.

### fold

Given interval
$I$
,
return
$\prod_{i \in I} A_i$
.

It takes
$O(\log n)$
time complexity.


