#include <numeric>
#include <regex>
#include <set>
#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"

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
    std::string s{"a;b,a;c,a;d"};
    std::regex delimiter(",");
    std::sregex_token_iterator it{s.begin(), s.end(), delimiter, -1};
    std::vector<std::string> words{it, {}};
    EXPECT_THAT(words, testing::ElementsAre("a;b", "a;c", "a;d"));

    std::string result = std::accumulate(
        words.begin(), words.end(), std::string(),
        [](const std::string &a, const std::string &b) -> std::string {
            return a + (a.length() > 0 ? "," : "") + "x;" + b;
        });

    EXPECT_EQ(result, "x;a;b,x;a;c,x;a;d");
}

TEST(FilterTest, SplitStrNoDelim) {
    std::string s{"a;b"};
    std::regex delimiter(",");
    std::sregex_token_iterator it{s.begin(), s.end(), delimiter, -1};
    std::vector<std::string> words{it, {}};
    EXPECT_THAT(words, testing::ElementsAre("a;b"));

    for (auto &w : words) {
        w = "x;" + w;
    }

    std::string result = std::accumulate(
        words.begin(), words.end(), std::string(),
        [](const std::string &a, const std::string &b) -> std::string {
            return a + (a.length() > 0 ? "," : "") + b;
        });

    EXPECT_EQ(result, "x;a;b");
}

TEST(FilterTest, SplitToSet) {
    std::string s{"a;b,a;c,a;d"};
    std::regex delimiter(",");
    std::sregex_token_iterator it{s.begin(), s.end(), delimiter, -1};
    std::set<std::string> words{it, {}};
    EXPECT_THAT(words, testing::UnorderedElementsAre("a;b", "a;c", "a;d"));
}

TEST(FilterTest, InitializerList) {
    std::set<std::string> vertices = {"a", "b", "c"};
    std::vector<std::pair<std::string, std::string>> edges = {{"a", "b"},
                                                              {"b", "c"}};

    std::map<std::string, std::map<std::vector<std::string>, std::string>>
        ids_to_properties;
    ids_to_properties["a"][{"x"}] = "y";
    ids_to_properties["b"][{"z", "w"}] = "123";
}
