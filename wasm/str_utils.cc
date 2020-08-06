#include <algorithm>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "str_utils.h"

std::vector<std::string>
str_split(const std::string &str, const std::string &delim, bool filter_empty) {
  std::regex re(delim);
  std::sregex_token_iterator it{str.begin(), str.end(), re, -1};

  std::vector<std::string> result{it, {}};
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
        return a + (a.length() > 0 ? std::string(delim) : "") + std::string(b);
      });
}

std::string str_join(std::vector<std::string_view> strs,
                     std::string_view delim) {
  return std::accumulate(
      strs.begin(), strs.end(), std::string(),
      [delim](const std::string &a, std::string_view b) -> std::string {
        return a + (a.length() > 0 ? std::string(delim) : "") + std::string(b);
      });
}

std::string str_join(std::vector<std::string> strs, std::string_view delim) {
  return std::accumulate(
      strs.begin(), strs.end(), std::string(),
      [delim](const std::string &a, const std::string &b) -> std::string {
        return a + (a.length() > 0 ? std::string(delim) : "") + b;
      });
}
