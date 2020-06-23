#include <regex>
#include <string>

#include "filter.pb.h"
#include "google/protobuf/util/json_util.h"
#include "gtest/gtest.h"

TEST(FilterTest, ParseName) {
  Config config;

  google::protobuf::util::JsonParseOptions options;
  options.case_insensitive_enum_parsing = true;
  options.ignore_unknown_fields = false;
  google::protobuf::util::JsonStringToMessage("{'name': 'productpage-v1'}",
                                              &config, options);

  EXPECT_EQ(config.name(), "productpage-v1");
}

TEST(FilterTest, RegexTest) {
  const std::regex base_regex(
      ".*,.*RecommendationService.*,.*ProductCatalogService.*");

  ASSERT_TRUE(
      std::regex_match("/product/3,/RecommendationService.ListRecommendations,/"
                       "ProductCatalogService.ListProducts",
                       base_regex));
  ASSERT_TRUE(
      std::regex_match("/cart,/RecommendationService.ListRecommendations,/"
                       "ProductCatalogService.ListProducts",
                       base_regex));

  ASSERT_FALSE(std::regex_match("/product/3/,/ProductCatalogService.GetProduct",
                                base_regex));
  ASSERT_TRUE(std::regex_match(
      "/cart/checkout,/hipstershop.RecommendationService/ListRecommendations,/"
      "hipstershop.ProductCatalogService/ListProducts",
      base_regex));
}