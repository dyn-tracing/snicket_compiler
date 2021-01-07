#include <string_view>
#include <vector>

#include "str_utils.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

TEST(StrSplitTest, Simple) {
  EXPECT_THAT(str_split("a-b-c,a-d,d-e", ","),
              testing::ElementsAre("a-b-c", "a-d", "d-e"));
  EXPECT_THAT(str_split("a-b-c-d-e", "-"),
              testing::ElementsAre("a", "b", "c", "d", "e"));
}

TEST(StrSplitTest, RegexOr) {
  EXPECT_THAT(
      str_split("a.x.y.z==123", absl::ByAnyChar(".="), absl::SkipEmpty()),
      testing::ElementsAre("a", "x", "y", "z", "123"));

  EXPECT_THAT(str_split("a.x.y.z == 123", absl::ByAnyChar(".= "),
                             absl::SkipEmpty()),
              testing::ElementsAre("a", "x", "y", "z", "123"));
}

TEST(StrSplitTest, Empty) {
  EXPECT_THAT(str_split("", ","), testing::ElementsAre(""));
  std::vector<std::string> result = str_split("", ",", absl::SkipEmpty());
  EXPECT_THAT(result, testing::IsEmpty());
}

TEST(StrJoinTest, Inputs) {
  EXPECT_EQ(str_join({"1", "2", "3"}, ","), "1,2,3");
  EXPECT_EQ(str_join(std::vector<std::string_view>{"1", "2", "3"}, ","),
            "1,2,3");
}
