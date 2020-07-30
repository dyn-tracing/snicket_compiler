#include <map>
#include <numeric>
#include <queue>
#include <regex>
#include <set>
#include <string>

#include "treenode.pb.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

// Returns true if TreeNode n2 has a subtree rooted at n2 which is ismorphic
// to n1.
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

TEST(GraphProtoTest, SingleTreeNode) {
  TreeNode n1;
  n1.set_id("n1");
  TreeNode n2;
  n2.set_id("n2");
  EXPECT_TRUE(isIsomorphic(n1, n2));
  n2.set_id("n1");
  EXPECT_TRUE(isIsomorphic(n1, n2));
}

TEST(GraphProtoTest, TreeNodeMatches) {
  TreeNode n1;
  n1.set_id("n1");
  n1.mutable_properties()->insert({"a", "x"});
  TreeNode n2;
  n2.set_id("n2");
  n2.mutable_properties()->insert({"a", "x"});
  n2.mutable_properties()->insert({"b", "y"});
  EXPECT_TRUE(isIsomorphic(n1, n2));
  n2.clear_properties();
  EXPECT_FALSE(isIsomorphic(n1, n2));
}

TEST(GraphProtoTest, ChildMatch) {
  TreeNode n1;
  n1.mutable_properties()->insert({"a", "x"});
  TreeNode *n1_child = n1.mutable_children()->Add();
  n1_child->mutable_properties()->insert({"b", "y"});
  TreeNode n2;
  n2.mutable_properties()->insert({"a", "x"});
  TreeNode *n2_child = n2.mutable_children()->Add();
  n2_child->mutable_properties()->insert({"b", "y"});
  n2_child = n2.mutable_children()->Add();
  n2_child->mutable_properties()->insert({"c", "z"});
  EXPECT_TRUE(isIsomorphic(n1, n2));
}

TEST(FilterTest, MapUpdate) {
  std::map<std::string, std::string> spans_to_headers;

  spans_to_headers.insert(std::make_pair("a", "1"));
  spans_to_headers.insert(std::make_pair("b", "2"));

  ASSERT_EQ(2, spans_to_headers.size());
  spans_to_headers.at("a") += "0";

  ASSERT_EQ(spans_to_headers.at("a"), "10");

  auto it = spans_to_headers.find("b");
  it->second += "00";

  ASSERT_EQ(spans_to_headers.at("b"), "200");

  it = spans_to_headers.find("c");
  ASSERT_EQ(it, spans_to_headers.end());
  spans_to_headers.insert(std::make_pair("c", "300"));

  ASSERT_EQ(spans_to_headers.at("c"), "300");

  ASSERT_EQ(3, spans_to_headers.size());

  it = spans_to_headers.find("c");
}

TEST(FilterTest, SplitStr) {
  std::string s{"a-b,a-c,a-d"};
  std::regex delimiter(",");
  std::sregex_token_iterator it{s.begin(), s.end(), delimiter, -1};
  std::vector<std::string> words{it, {}};
  EXPECT_THAT(words, testing::ElementsAre("a-b", "a-c", "a-d"));

  std::string result = std::accumulate(
      words.begin(), words.end(), std::string(),
      [](const std::string &a, const std::string &b) -> std::string {
        return a + (a.length() > 0 ? "," : "") + "x-" + b;
      });

  EXPECT_EQ(result, "x-a-b,x-a-c,x-a-d");
}

TEST(FilterTest, SplitStrTreeNodelim) {
  std::string s{"a-b"};
  std::regex delimiter(",");
  std::sregex_token_iterator it{s.begin(), s.end(), delimiter, -1};
  std::vector<std::string> words{it, {}};
  EXPECT_THAT(words, testing::ElementsAre("a-b"));

  for (auto &w : words) {
    w = "x-" + w;
  }

  std::string result = std::accumulate(
      words.begin(), words.end(), std::string(),
      [](const std::string &a, const std::string &b) -> std::string {
        return a + (a.length() > 0 ? "," : "") + b;
      });

  EXPECT_EQ(result, "x-a-b");
}

TEST(FilterTest, SplitToSet) {
  std::string s{"a-b,a-c,a-d"};
  std::regex delimiter(",");
  std::sregex_token_iterator it{s.begin(), s.end(), delimiter, -1};
  std::set<std::string> words{it, {}};
  EXPECT_THAT(words, testing::UnorderedElementsAre("a-b", "a-c", "a-d"));
}