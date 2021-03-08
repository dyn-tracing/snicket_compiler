#include <algorithm>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "str_utils.h"

// trim from start
static inline std::string &ltrim(std::string &s) {
    s.erase(s.begin(),
            std::find_if(s.begin(), s.end(),
                         std::not1(std::ptr_fun<int, int>(std::isspace))));
    return s;
}

// trim from end
static inline std::string &rtrim(std::string &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(),
                         std::not1(std::ptr_fun<int, int>(std::isspace)))
                .base(),
            s.end());
    return s;
}

// trim from both ends
static inline std::string &trim(std::string &s) { return ltrim(rtrim(s)); }

std::vector<std::string>
str_split(const std::string &str, const std::string &delim, bool filter_empty) {
    std::regex re(delim);
    std::sregex_token_iterator first{str.begin(), str.end(), re, -1}, last;

    std::vector<std::string> tmp = {first, last};
    std::vector<std::string> result;
    for (auto &split_result : tmp) {
        result.push_back(trim(split_result));
    }
    if (filter_empty) {
        result.erase(std::remove_if(result.begin(), result.end(),
                                    [](std::string s) { return s.empty(); }),
                     result.end());
    }

    return result;
}

std::string str_join(std::initializer_list<std::string_view> strs,
                     std::string_view delim) {
    return std::accumulate(
        strs.begin(), strs.end(), std::string(),
        [delim](const std::string &a, std::string_view b) -> std::string {
            return a + (a.length() > 0 ? std::string(delim) : "") +
                   std::string(b);
        });
}

std::string str_join(std::vector<std::string_view> strs,
                     std::string_view delim) {
    return std::accumulate(
        strs.begin(), strs.end(), std::string(),
        [delim](const std::string &a, std::string_view b) -> std::string {
            return a + (a.length() > 0 ? std::string(delim) : "") +
                   std::string(b);
        });
}

std::string str_join(std::vector<std::string> strs, std::string_view delim) {
    return std::accumulate(
        strs.begin(), strs.end(), std::string(),
        [delim](const std::string &a, const std::string &b) -> std::string {
            return a + (a.length() > 0 ? std::string(delim) : "") + b;
        });
}
