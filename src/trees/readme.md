q: distinguish between graph, tree, free tree and forest

a: free tree is a acyclic, connected, undirected graph. a tree commonly refers
to a free tree. a forest has a set of trees.


q: given an undirected graph, what effect does adding and removing edges have on
connected and acyclic properties and on number of pathways? where is a free tree
in this?

a: adding edges adds pathways. if it adds two pathways, there is a cycle.
acyclic has at most 1 path between every node. removing edges removes pathways.
if there is no pathway, it is unconnected. connected means at least 1 path. if
adding an edge means becoming cyclic, or if removing an edge means becoming
unconnected, then there is exactly 1 path between every node. this type of graph
is called a free tree. 


q: distinguish between free tree, rooted tree, and ordered tree

a: a rooted tree is a free tree with a distinguished vertex. an ordered tree has
ordered children at nodes.


q: given a rooted tree: distinguish between root, node, ancestor, descendent,
proper ancestor, proper descendent, "subtree rooted at", parent, child, sibling,
leaf, external node, non-leaf, internal node


q: given a rooted tree: distinguish between degree, depth, level, height

a: nodes have degree, depth, and height. degree is number of children. depth is
number of edges between node and root. the height of a node is the is the number
of edges in the longest path to its leafs. a tree has levels and a height. a
level is the set of nodes with the same depth. the height of a tree is the
height of its root node.


q: degree of a node in a rooted tree vs in a free tree vs in an undirected graph

a: in a rooted tree, its number of children. in an undirected graph and free
tree, its number of adjacent nodes.


q: distinguish between: binary tree, empty tree, null tree, ordered tree, full
binary tree, positional tree, k-ary tree, and complete k-ary tree

a:

a binary tree is an empty set or a set containing three disjoint sets: a set
with a single root node, a left binary subtree, and a right binary subtree. a
binary tree that's an empty set is an empty or null tree.

now, suppose we have a binary tree with a node with a degree of 1. the child
could be a left node or a right node, and the resulting trees would be
different. however, as an ordered tree, it must be the first node, so there is
no distinction. therefore, a binary tree is not an ordered tree.

it'd be useful if it were. a full binary tree is one that for every node, the
degree is 0 or 2. we can represent a binary tree with a full binary tree by
converting every empty tree and replacing it with a node. the original binary
tree is encoded in the internal nodes of the resulting tree.

now, every child can be labeled with a position: the ith child. this makes it a
positional tree. a k-ary tree is a positional tree with every node missing the
k+1th and up children. a binary tree is k equals 2. representing a binary tree
with a k-ary tree allows algorithms that apply to k-ary trees apply to binary
trees, even non full ones.

a complete k-ary tree is one wherver (1) all leaves have the same depth, and (2)
all internal nodes have the same degree.
