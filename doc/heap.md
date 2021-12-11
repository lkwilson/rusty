q: how do the internals of a priority queue work, create, insert, remove, set
values. hows it relate to sorting.

a: heapifying is easy if the children are heaps, so you start from the second to
last layer where the children only have 1 node. once every subtree on that level
is a heap, the parent level can repeat knowing its children are heaps.
heapifying just takes a node and its children, finds the largest swaps with it.
the swap breaks the heap property of the child, so the child then gets
heapified. it could be as long as the height of the tree, so this heapify a
single node process is O(log(n))

you don't have to heapify half of the nodes, and half of the remainder has a
height of 1. half of the remainder of that has a height of 2. while the
algorithm seems like n heapify calls, log(n) each, it is provably just O(n) in
total because the O(log(n)) calls has a very low average n.

once you have a heap, insert and remove need to maintain it. The only way to get
elements in and out is through the back.

to insert, push a value on. now the parent of that node might not be a heap, so
we need to heapify its parent. then the parent of the parent may not be a heap,
so it needs to be heapified. repeat. best case O(1), worst case O(log(n)).

to remove, we need to swap some value, usually the root, with the end and then
pop it off. however, the heap is ruined by doing this, so you have to heapify
from where you swapped to. the swapped node either swaps with its parents, or
swaps with a child. usually, it's swapped with the root node, so you only have
to call the heapify with the assumption that children are still heaps.

there's a similar process for changing values except without the swap pop part.
it floats up or down.

pop off a priority queue to get a sorted version of the array. O(n) to build the
heap, and O(log(n)) to remove elements, so an nlogn sorting algorithm. Though
apparently quick sort is still better in practice.
