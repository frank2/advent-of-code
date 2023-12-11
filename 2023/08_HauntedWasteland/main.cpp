#include <algorithm>
#include <cstdint>
#include <iostream>
#include <sstream>
#include <map>
#include <memory>
#include <numeric>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

using DesertMap = std::map<std::string,std::pair<std::string,std::string>>;
   
std::pair<std::string,DesertMap> read_input(std::istream &stream)
{
   std::string line, directions;
   DesertMap result;

   std::getline(stream, directions);
   std::getline(stream, line);
      
   while (std::getline(stream, line))
   {
      std::string label, eq, left, right;
      std::stringstream line_stream(line);
      line_stream >> label >> eq >> left >> right;
      result[label] = {left.substr(1, 3), right.substr(0, 3)};
   }
                                                       
   return {directions, result};
}

std::size_t traverse(const std::string directions, const DesertMap &map)
{
   auto label = std::string("AAA");
   std::size_t result = 0;

   while (label != "ZZZ")
   {
      char path = directions[result % directions.size()];

      if (path == 'L')
         label = map.at(label).first;
      else
         label = map.at(label).second;

      ++result;
   }

   return result;
}

std::size_t traverse_another(const std::string directions, const DesertMap &map)
{
   std::vector<std::string> starting_points;

   for (auto pair : map)
      if (pair.first[2] == 'A')
         starting_points.push_back(pair.first);

   std::vector<std::size_t> modular;

   for (auto entrypoint : starting_points)
   {
      auto label = entrypoint;

      std::size_t local_result = 0;

      while (label[2] != 'Z')
      {
         if (directions[local_result % directions.size()] == 'L')
            label = map.at(label).first;
         else
            label = map.at(label).second;

         ++local_result;
      }

      modular.push_back(local_result);
   }

   std::sort(modular.begin(), modular.end());
   bool all_found = false;
   std::size_t result = modular.back();

   while (!all_found)
   {
      all_found = true;
      
      for (auto module : modular)
      {
         if (result % module != 0)
         {
            all_found = false;
            break;
         }
      }

      if (!all_found)
         result += modular.back();
   }
         
   return result;
}

int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);

   std::cout << "part 1: " << traverse(puzzle_input.first, puzzle_input.second) << std::endl;
   std::cout << "part 2: " << traverse_another(puzzle_input.first, puzzle_input.second) << std::endl;
   
   return 0;
}
