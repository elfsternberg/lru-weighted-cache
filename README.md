![Language: Rust](https://img.shields.io/badge/language-Rust-green.svg)
![Topic: Memory Caching](https://img.shields.io/badge/topic-Memory_Caching-yellow.svg)
![Status: In Progress](https://img.shields.io/badge/status-In_Progress-yellow.svg)

# LRU Cache with Sized Eviction Criteria

A [Rust](https://www.rust-lang.org) library providing a [Weight-Aware Least
Recently Used
Cache](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU)
with a ceiling and eviction criteria based on the "weight" of inserted
objects.

## What is it?

An implementation of an LRU Cache that will never exceed a certain
weight, where "weight" is defined by the client code.  The cache
supports 'get', 'insert', and 'remove' operations, all of which are
approximately O(ln(n)).  We use HashMap under the covers.  The
difference between this implentation and reference implementation is
that the *values* must implement the `Weigted` trait, which has one
function, `weight()`.  The container takes two size arguments, one the
largest legal size of an individual item, the other the number of
maximally sized objects the cache can hold.

The sizes are *arbitrary* and don't actually mean anything.  The intent
is to track the maximum memory used by the container, but that intent is
applied by the developer when the `weight()` function is defined, not by
the code.

For example, if `String.len()` is your weight, your `max_weight` is 20,
and your `max_count` is 5, then the total weight of the cache is 100:
the cache could hold 5 strings of length 20, but it could also hold 10
strings of length 10, or 25 strings of length 4, and so on.  It could
not, however, hold 4 strings of length 25: the `insert()` method will
*reject* an object above the `max_weight`.

A client could implement a `weight()` function that always returns 1,
and set the `max_item_size` to 1; in that case this function behaves
exactly like Rust's default LRU cache.

Also: I wrote this as part of
[Monologued](https://elfsternberg.github.com/monologued), so it's part
of my self-education into Rust.

## Status

It seems to be working and the tests seem to catch everything, including
one very ugly dereferencing bug I found earlier.  We'll know more when I
start beating this thing up using Monologue.

## LICENSE

This implementation is Copyright [Elf
M. Sternberg](https://elfsternberg.com) (c) 2019, and licensed with the
Mozilla Public License vers. 2.0.  A copy of the license file is
included in the root folder.
