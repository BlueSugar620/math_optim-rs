# Static Range Inversions Query

## Definition

Let
$T$
be a ordered set.


### construct

Given
$A = (A_i)_{i = 0}^{n - 1}$
,
struct static range inversions query.

It takes
$\langle O(n \sqrt{n}), O(n \sqrt{n}) \eangle$
complexity.

### calculate inversions

Given interval
$I$
,
return the inversions of
$(A_i)_{i \in I}$
.

It takes
$O(\sqrt{n})$
time complexity.
