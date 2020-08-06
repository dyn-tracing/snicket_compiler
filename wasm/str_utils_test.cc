#include <string_view>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

#include "str_utils.h"

TEST(StrSplitTest, Simple) {
  EXPECT_THAT(str_split("a-b-c,a-d,d-e", ","),
              testing::ElementsAre("a-b-c", "a-d", "d-e"));
  EXPECT_THAT(str_split("a-b-c-d-e", "-"),
              testing::ElementsAre("a", "b", "c", "d", "e"));
}

TEST(StrSplitTest, RegexOr) {
  EXPECT_THAT(str_split("a.x.y.z==123", "[.]|(==)"),
              testing::ElementsAre("a", "x", "y", "z", "123"));

  EXPECT_THAT(
      str_split("a.x.y.z == 123", R"([.]|(==)|(\s+))", /*filter_empty=*/true),
      testing::ElementsAre("a", "x", "y", "z", "123"));
}

TEST(StrSplitTest, Empty) {
  EXPECT_THAT(str_split("", ","), testing::ElementsAre(""));
  EXPECT_THAT(str_split("", ",", /*filter_empty=*/true), testing::IsEmpty());
}

TEST(StrJoinTest, Inputs) {
  EXPECT_EQ(str_join({"1", "2", "3"}, ","), "1,2,3");
  EXPECT_EQ(str_join(std::vector<std::string_view>{"1", "2", "3"}, ","),
            "1,2,3");
}