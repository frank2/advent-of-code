#include <cstdint>
#include <iostream>
#include <map>
#include <numeric>
#include <string>
#include <vector>

using CubeRule = std::map<std::string,std::uint64_t>;
using CubeGame = std::vector<CubeRule>;

std::vector<CubeGame> read_input(std::istream &stream)
{
   std::vector<CubeGame> result;
   
   for (std::string line; std::getline(stream, line);)
   {
      // split on ':' to get the Game chunk and the Rules chunk, discard the game chunk
      std::string str_rules = line.substr(line.find(": ")+2);
      
      // split on ';' to get the game draws
      std::vector<std::string> vec_str_rules;
      std::size_t boundary = 0;
      CubeGame game;

      do
      {
         std::size_t new_boundary = str_rules.find("; ", boundary);

         if (new_boundary == std::string::npos)
         {
            vec_str_rules.push_back(str_rules.substr(boundary));
            boundary = new_boundary;
         }
         else
         {
            vec_str_rules.push_back(str_rules.substr(boundary, new_boundary-boundary));
            boundary = new_boundary+2;
         }
      } while (boundary != std::string::npos);

      for (auto str_rule : vec_str_rules)
      {
         boundary = 0;

         CubeRule rules;

         do
         {
            // split on ',' to get the colors
            std::size_t new_boundary = str_rule.find(", ", boundary);
            std::string rule_chunk;

            if (new_boundary == std::string::npos)
            {
               rule_chunk = str_rule.substr(boundary);
               boundary = new_boundary;
            }
            else
            {
               rule_chunk = str_rule.substr(boundary, new_boundary-boundary);
               boundary = new_boundary+2;
            }

            // split on ' ' to get the number and the color
            std::size_t split_point = rule_chunk.find(" ");
            rules[rule_chunk.substr(split_point+1)] = std::stol(rule_chunk.substr(0, split_point));
         } while (boundary != std::string::npos);

         game.push_back(rules);
      }
         
      result.push_back(game);
   }

   return result;
}

std::vector<std::size_t> possible_games(const std::vector<CubeGame> &games, const CubeRule &rules)
{
   std::vector<std::size_t> result;
   std::size_t game_id = 1;

   for (auto game : games)
   {
      bool possible = true;

      for (auto rule : game)
      {
         for (auto cube : rule)
         {
            if (cube.second > rules.at(cube.first))
            {
               possible = false;
               break;
            }
         }

         if (possible == false)
            break;
      }

      if (possible)
         result.push_back(game_id);

      ++game_id;
   }

   return result;
}

std::uint64_t cube_power(const std::vector<CubeGame> &games)
{
   std::uint64_t result = 0;

   for (auto game : games)
   {
      CubeRule count = {{"red", 0}, {"blue", 0}, {"green", 0}};;

      for (auto rule : game)
      {
         for (auto cube : rule)
         {
            if (count[cube.first] < cube.second)
               count[cube.first] = cube.second;
         }
      }

      result += count["red"] * count["green"] * count["blue"];
   }

   return result;
}      

int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);
   CubeRule rules = {{"red", 12}, {"green", 13}, {"blue", 14}};
   auto possible = possible_games(puzzle_input, rules);
   auto powers = cube_power(puzzle_input);

   std::cout << "part 1: " << std::reduce(possible.begin(), possible.end()) << std::endl;
   std::cout << "part 2: " << powers << std::endl;
   
   return 0;
}
