# LRU Cache with Sized Eviction Criteria

A least-recently-used cache with a memory ceiling and eviction criteria
based on the size of inserted objects.

# What is it?

An implementation of an LRU Cache.  The cache supports 'get', 'put', and
pop' operations, all of which are approximately O(1).  Uses Rust's Robin
Hood HashMap under the covers.  The difference between this one and the
one in nightly is that the *values* must be Sized (that is, they must
implement a len() function), the HashMap must be instantiated with a
pair of usizes: one the largest object the HashMap will accept, and two
the maximum number of those objects the HashMap will handle.

This cache is meant to support a simple text cache, with the caveat that
the over memory usage has a ceiling: Maximum_Entry_Size *
Maximum_Number_Of_Maximal_Entries.  If your largest entry is half the
size of the maximum entry size, then you could store twice as many.
Provided an object to be inserted is below the maximum entry size,
insertion will systematically evict least-used entries until there is
sufficient room for the object in question.

Also: I wrote this as part of
[Monologued](https://elfsternberg.github.com/monologued), so it's part
of my self-education into Rust.
