#ifndef STR_UTILS_H
#define STR_UTILS_H

#include <initializer_list>
#include <string>
#include <string_view>
#include <vector>

std::vector<std::string> str_split(const std::string &str,
                                   const std::string &delim,
                                   bool filter_empty = false);

std::string str_join(std::initializer_list<std::string_view> strs,
                     std::string_view delim);

std::string str_join(std::vector<std::string_view> strs,
                     std::string_view delim);

std::string str_join(std::vector<std::string> strs, std::string_view delim);

#endif // STR_UTILS_H