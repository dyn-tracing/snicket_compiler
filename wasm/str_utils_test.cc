#include <string_view>
#include <vector>

#include "absl/strings/str_split.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

#include "str_utils.h"

TEST(StrSplitTest, Simple) {
  EXPECT_THAT(absl::StrSplit("a-b-c,a-d,d-e", ","),
              testing::ElementsAre("a-b-c", "a-d", "d-e"));
  EXPECT_THAT(absl::StrSplit("a-b-c-d-e", "-"),
              testing::ElementsAre("a", "b", "c", "d", "e"));
}

TEST(StrSplitTest, RegexOr) {
  EXPECT_THAT(
      absl::StrSplit("a.x.y.z==123", absl::ByAnyChar(".="), absl::SkipEmpty()),
      testing::ElementsAre("a", "x", "y", "z", "123"));

  EXPECT_THAT(absl::StrSplit("a.x.y.z == 123", absl::ByAnyChar(".= "),
                             absl::SkipEmpty()),
              testing::ElementsAre("a", "x", "y", "z", "123"));
}

TEST(StrSplitTest, Empty) {
  EXPECT_THAT(absl::StrSplit("", ","), testing::ElementsAre(""));
  std::vector<std::string> result = absl::StrSplit("", ",", absl::SkipEmpty());
  EXPECT_THAT(result, testing::IsEmpty());
}

TEST(StrJoinTest, Inputs) {
  EXPECT_EQ(str_join({"1", "2", "3"}, ","), "1,2,3");
  EXPECT_EQ(str_join(std::vector<std::string_view>{"1", "2", "3"}, ","),
            "1,2,3");
}
