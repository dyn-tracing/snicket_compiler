#include <queue>

#include "treenode.pb.h"

// Returns true if TreeNode n2 has a subtree rooted at n2 which is ismorphic
// to n1.
bool isIsomorphic(const TreeNode &n1, const TreeNode &n2);

// Returns true if TreeNode n2 has a subtree which is isomorphic to n1.
bool isSubgraphIsomorphic(const TreeNode &n1, const TreeNode &n2);
