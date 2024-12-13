# Segment Tree

## Definition

Let
$(M, \cdot)$
be a monoid, and
$A = (A_i)_{i = 0}^{n - 1} \subseteq M$
be a sequence on the monoid.

The following values are calculated at hitgh speed.

### construct

Given the sequence 
$A = (A_i)_{i = 0}^{n - 1}$
,
struct segment tree of
$A$
.

It takes
$\langle O(n \log n), O(n) \rangle$
complexity.

### point-at get

Given
$i \in [0, n)$
,
return
$A_i$
.

It takes
$O(1)$
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

