#include "treenode_util.h"

#include "treenode.pb.h"

bool isIsomorphic(const TreeNode &n1, const TreeNode &n2) {
  for (const auto &pair : n1.properties()) {
    if ((!n2.properties().contains(pair.first)) ||
        (n2.properties().at(pair.first) != pair.second)) {
      return false;
    }
  }

  if (n1.children().size() > n2.children().size()) {
    return false;
  }

  // NOTE: Doesn't handle the case where other_child is isomorphic to multiple
  // child
  for (int i = 0; i < n1.children().size(); ++i) {
    const auto &child = n1.children()[i];
    bool has_isomorphic_node = false;
    for (int j = 0; j < n2.children().size(); ++j) {
      const auto &other_child = n2.children()[j];
      if (isIsomorphic(child, other_child)) {
        has_isomorphic_node = true;
      }
    }
    if (!has_isomorphic_node) {
      return false;
    }
  }

  return true;
}

// Returns true if TreeNode n2 has a subtree which is isomorphic to n1.
bool isSubgraphIsomorphic(const TreeNode &n1, const TreeNode &n2) {
  std::queue<const TreeNode *> candidates;
  candidates.push(&n2);

  while (candidates.size() > 0) {
    const TreeNode *node = candidates.front();
    if (isIsomorphic(n1, *node)) {
      return true;
    }

    for (const auto &child : node->children()) {
      candidates.push(&child);
    }
    candidates.pop();
  }

  return false;
}
