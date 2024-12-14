# Disjoint Set Union

## Definition

### construct

Given
$n$
,
initilize the set 
$U = \lbrace \lbrace i \rbrace : i = 0, \cdots,n - 1  \rbrace$
.
This implies that disjoint set union.

### root

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return the representation of
$i$ 's
root.

If two points are in same set, return the same.

### unite

Given
$i, j \in \lbrace 0, \cdots, n - 1 \rbrace$
,
unite the groups which each
$i, j$
belong to.

### same

Given
$i, j \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return whether
$i$
and
$j$
are in the same set.

### size

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return the size of set which
$i$
belong to.

### count

Return the number of set.

# Disjoint Set Union on Monoid

## Definition

Let
$(M, \cdot)$
be a commutative monoid.

### construct

Given
$n$
,
initilize the set 
$U = \lbrace \lbrace i \rbrace : i = 0, \cdots,n - 1  \rbrace$
and
$f = e$
.
This implies that disjoint set union and the function characterizes the set, which means that the points which belong the same return the same.

### root

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return the representation of
$i$ 's
root.

If two points are in same set, return the same.

### value

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return
$f(i)$
.

### point-at update value

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
and
$x \in M$
,
update
$f(i) = x$
.

### unite

Given
$i, j \in \lbrace 0, \cdots, n - 1 \rbrace$
,
unite the groups which each
$i, j$
belong to.

### same

Given
$i, j \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return whether
$i$
and
$j$
are in the same set.

### size

Given
$i \in \lbrace 0, \cdots, n - 1 \rbrace$
,
return the size of set which
$i$
belong to.

### count

Return the number of set.


# Disjoint Set Union with relation

## to be later...

I cannot understand how set should i use......
