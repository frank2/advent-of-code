#include <cstdint>
#include <iostream>
#include <map>
#include <optional>
#include <string>
#include <vector>

std::vector<std::string> read_input(std::istream &stream)
{
   std::vector<std::string> result;
   
   for (std::string line; std::getline(stream, line);)
   {
      result.push_back(line);
   }

   return result;
}

std::uint64_t count_bookends_digit(const std::vector<std::string> &input)
{
   std::uint64_t result = 0;
   
   for (auto line : input)
   {
      std::vector<char> digits;
      
      for (auto c : line)
      {
         if (c >= '0' && c <= '9')
            digits.push_back(c);
      }

      if (digits.size() == 0)
         continue;

      char left = digits.front();
      char right = digits.back();
      result += (left - '0') * 10 + (right - '0');
   }

   return result;
}

std::uint64_t count_bookends_word(const std::vector<std::string> &input)
{
   std::uint64_t result = 0;
   std::map<char, std::map<char, std::string>> number_tree = {
      {'o', {{'n', "one"}}},
      {'t', {{'w', "two"}, {'h', "three"}}},
      {'f', {{'o', "four"}, {'i', "five"}}},
      {'s', {{'i', "six"}, {'e', "seven"}}},
      {'e', {{'i', "eight"}}},
      {'n', {{'i', "nine"}}}};
   std::map<std::string, std::uint64_t> number_map = {
      {"one", 1},
      {"two", 2},
      {"three", 3},
      {"four", 4},
      {"five", 5},
      {"six", 6},
      {"seven", 7},
      {"eight", 8},
      {"nine", 9}};
   std::optional<char> level1_match, level2_match;
   std::optional<std::string> level3_match;
   
   for (auto line : input)
   {
      std::vector<std::uint64_t> digits;
      std::size_t index = 0;
      std::size_t match_index = 0;

      level1_match = std::nullopt;
      level2_match = std::nullopt;
      level3_match = std::nullopt;
      
      while (index < line.size())
      {
         char c = line[index++];

         if (c >= '0' && c <= '9')
         {
            digits.push_back(c - '0');
            level1_match = std::nullopt;
            level2_match = std::nullopt;
            level3_match = std::nullopt;
            match_index = index;
            continue;
         }

         if (!level1_match.has_value())
         {
            match_index = index;
            
            if (auto search = number_tree.find(c); search != number_tree.end())
            {
               level1_match = c;
            }

            continue;
         }

         if (!level2_match.has_value())
         {
            if (auto search = number_tree[*level1_match].find(c); search != number_tree[*level1_match].end())
            {
               level2_match = c;
               continue;
            }
            else
            {
               index = match_index;
               level1_match = std::nullopt;
               continue;
            }
         }

         if (!level3_match.has_value())
            level3_match = number_tree[*level1_match][*level2_match];

         std::size_t partial_index = index-match_index;
         char mc = level3_match->at(partial_index);

         if (c == mc)
         {
            if (partial_index+1 == level3_match->size())
            {
               digits.push_back(number_map[*level3_match]);
               index = match_index;
               level1_match = std::nullopt;
               level2_match = std::nullopt;
               level3_match = std::nullopt;
            }

            continue;
         }
         else
         {
            index = match_index;
            level1_match = std::nullopt;
            level2_match = std::nullopt;
            level3_match = std::nullopt;
         }
      }
      
      if (digits.size() == 0)
         continue;

      auto left = digits.front();
      auto right = digits.back();
      result += left * 10 + right;
   }

   return result;
}

int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);
   std::cout << "part 1: " << count_bookends_digit(puzzle_input) << std::endl;
   std::cout << "part 2: " << count_bookends_word(puzzle_input) << std::endl;

   return 0;
}
