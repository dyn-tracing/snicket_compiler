#include <algorithm>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "str_utils.h"

// trim from start (in place)
static inline void ltrim(std::string &s) {
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
                return !std::isspace(ch);
            }));
}

// trim from end (in place)
static inline void rtrim(std::string &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(),
                         [](unsigned char ch) { return !std::isspace(ch); })
                .base(),
            s.end());
}

// trim from both ends (in place)
static inline void trim(std::string &s) {
    ltrim(s);
    rtrim(s);
}

// trim from start (copying)
static inline std::string ltrim_copy(std::string s) {
    ltrim(s);
    return s;
}

// trim from end (copying)
static inline std::string rtrim_copy(std::string s) {
    rtrim(s);
    return s;
}

// trim from both ends (copying)
static inline std::string trim_copy(std::string s) {
    trim(s);
    return s;
}

std::vector<std::string>
str_split(const std::string &str, const std::string &delim, bool filter_empty) {
    std::regex re(delim);
    std::sregex_token_iterator first{str.begin(), str.end(), re, -1}, last;

    std::vector<std::string> result = {first, last};
    // trim in place
    for (auto &split_result : result) {
        trim(split_result);
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
