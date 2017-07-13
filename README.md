# LRU Cache with Sized Eviction Criteria

A least-recently-used cache with a memory ceiling and eviction criteria
based on the size of inserted objects.

# What is it?

An implementation of an LRU Cache.  The cache supports 'get', 'put', and
pop' operations, all of which are approximately O(1).  Uses Rust's Robin
Hood HashMap under the covers.  The difference between this implentation
and reference implementation is that the *values* must implement the
ArbSize trait, which has one function, size().  The container takes two
size arguments, one the largest legal size of an individual item, the
other the number of maximally sized objects the cache can hold.

The sizes are *arbitrary* and don't actually mean anything.  The intent
is to track the maximum memory used by the container, but that intent is
applied by the developer when the size() function is defined, not by the
code.

Also: I wrote this as part of
[Monologued](https://elfsternberg.github.com/monologued), so it's part
of my self-education into Rust.
