# Idea
A graph is bipartite means that it can be divided into two sets of nodes, such that all edges connect a node from one set to a node from the other set. Let's call this set A and B. The simplest way to understand the idea is to say that this node in this graph cannot be in the same set as his neighbor.

We can create the two sets as EVEN and ODD. We start from a node and we put it in the EVEN set. Then we put all his neighbors in the ODD set. Then we put all their neighbors in the EVEN set, and so on. If we find a node that is already in a set, and we try to put it in the other set, then we have a conflict and the graph is not bipartite.
