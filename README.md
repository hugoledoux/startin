# rustin

A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
This is used mostly for the modelling of terrains.

The algorithm used is an incremental insertion based on flips, and the data structure is a cheap implementation of the star-based structure defined in [Blandford et al. (2005)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array (`Vec`) and not in an optimised blob like they did.
Still, it results is a pretty fast library it seems.

Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html)), so the library is robust and shouldn't crash (touch wood). 

I made this in Rust because I wanted to learn Rust.


