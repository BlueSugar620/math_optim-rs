# Lazy Segment Tree

## Definition

Let
$(M, \cdot_M)$
and
$(S, \cdot_S)$
be monoids, it is a
$S$
monoid action to
$M$
.

### construct

Given the sequence
$A = (A_i)_{i = 0}^{n - 1} \subseteq M$
,
struct lazy segment tree of
$A$.

It takes
$\langle O(n \log n), O(n) \rangle$
complexity.

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
$O(log n)$
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

### fold action

Given interval
$I$
and
$s \in S$
,
for all
$i \in I$
update
$A_i$
to
$s \cdot A_i$
.

It takes
$O(\log n)$
time complexity.
