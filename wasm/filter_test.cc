#include <numeric>
#include <regex>
#include <set>
#include <string>

#include "treenode.pb.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

// Returns true if TreeNode n1 property set is a subset of n2's property set
// and n1's children is a subset of n2's children set. Their ids can be
// different.
bool property_child_subset(const TreeNode &n1, const TreeNode &n2) {
  for (const auto &pair : n1.properties()) {
    if (!(n2.properties().contains(pair.first)) ||
        (n2.properties().at(pair.first) != pair.second)) {
      return false;
    }
  }

  if (n1.children().size() > n2.children().size()) {
    return false;
  }

  for (const auto &child : n1.children()) {
    bool check = false;
    for (const auto &other_child : n2.children()) {
      if (property_child_subset(child, other_child)) {
        check = true;
      }
    }
    if (!check) {
      return false;
    }
  }

  return true;
}

TEST(GraphProtoTest, SingleTreeNode) {
  TreeNode n1;
  n1.set_id("n1");
  TreeNode n2;
  n2.set_id("n2");
  EXPECT_TRUE(property_child_subset(n1, n2));
  n2.set_id("n1");
  EXPECT_TRUE(property_child_subset(n1, n2));
}

TEST(GraphProtoTest, TreeNodeMatches) {
  TreeNode n1;
  n1.set_id("n1");
  n1.mutable_properties()->insert({"a", "x"});
  TreeNode n2;
  n2.set_id("n2");
  n2.mutable_properties()->insert({"a", "x"});
  n2.mutable_properties()->insert({"b", "y"});
  EXPECT_TRUE(property_child_subset(n1, n2));
  n2.clear_properties();
  EXPECT_FALSE(property_child_subset(n1, n2));
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
  EXPECT_TRUE(property_child_subset(n1, n2));
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