#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

enum Direction
{
   LEFT = 0,
   TOP,
   RIGHT,
   BOTTOM,
   MAX
};

using Forest = std::vector<std::vector<std::int8_t>>;
using Visibility = std::vector<std::vector<bool>>;

Visibility get_visibility(const Forest &forest)
{
   Visibility result = std::vector<std::vector<bool>>(forest.size(), std::vector<bool>(forest[0].size(), false));

   for (std::size_t dir=Direction::LEFT; dir<Direction::MAX; ++dir)
   {
      auto direction = static_cast<Direction>(dir);
      
      for (std::size_t y=0; y<forest.size(); ++y)
      {
         std::int8_t max = -1;

         for (std::size_t x=0; x<forest[0].size(); ++x)
         {
            std::size_t x_index, y_index;
            
            switch (direction)
            {
            case Direction::LEFT:
               x_index = x; y_index = y;
               break;

            case Direction::TOP:
               x_index = y; y_index = x;
               break;

            case Direction::RIGHT:
               x_index = forest.size()-1-x; y_index = y;
               break;

            case Direction::BOTTOM:
               x_index = y; y_index = forest.size()-1-x;
               break;
            }

            if (forest[y_index][x_index] > max || x_index == 0 || x_index == forest[0].size()-1 || y_index == 0 || y_index == forest.size()-1)
            {
               result[y_index][x_index] = true;
               max = forest[y_index][x_index];
            }
         }
      }
   }

   return result;
}

std::size_t scenic_score(const Forest &forest, const Visibility &visibility)
{
   std::size_t max_score = 0;

   if (forest.size() == 0 || forest[0].size() == 0)
      return max_score;
   
   for (std::intptr_t y=1; y<static_cast<std::intptr_t>(forest.size())-1; ++y)
   {
      for (std::intptr_t x=1; x<static_cast<std::intptr_t>(forest[0].size())-1; ++x)
      {
         if (!visibility[y][x]) { continue; }

         std::size_t score = 1;
         std::int8_t tree = forest[y][x];
         
         for (std::size_t dir=Direction::LEFT; dir<Direction::MAX; ++dir)
         {
            auto direction = static_cast<Direction>(dir);
            std::intptr_t offset = -1;
            std::size_t visible = 0;
            bool valid = true;

            if (direction == Direction::RIGHT || direction == Direction::BOTTOM)
               offset *= -1;

            while (valid)
            {
               std::intptr_t x_index, y_index;
               
               switch (direction)
               {
               case Direction::LEFT:
               case Direction::RIGHT:
                  x_index = x+offset; y_index = y;
                  break;

               case Direction::TOP:
               case Direction::BOTTOM:
                  x_index = x; y_index = y+offset;
                  break;
               }

               switch (direction)
               {
               case Direction::LEFT:
               case Direction::TOP:
                  --offset;
                  break;

               case Direction::RIGHT:
               case Direction::BOTTOM:
                  ++offset;
                  break;
               }

               ++visible;
               
               if (forest[y_index][x_index] >= tree)
                  break;
               
               switch (direction)
               {
               case Direction::LEFT:
               case Direction::RIGHT:
                  valid = (x_index > 0 && x_index < static_cast<std::intptr_t>(forest[0].size())-1);
                  break;

               case Direction::TOP:
               case Direction::BOTTOM:
                  valid = (y_index > 0 && y_index < static_cast<std::intptr_t>(forest.size())-1);
                  break;
               }
            } 

            if (visible > 0)
               score *= visible;
         }
         
         if (score > max_score)
            max_score = score;
      }
   }

   return max_score;
}
         
Forest parse_forest()
{
   Forest result;
   std::string line;

   while (std::getline(std::cin, line) && line.size() > 0)
   {
      std::vector<std::int8_t> row;

      for (auto c : line)
         row.push_back(static_cast<std::int8_t>(c - '0'));

      result.push_back(row);
   }

   return result;
}

int main(int argc, char *argv[])
{
   auto forest = parse_forest();
   auto visibility = get_visibility(forest);
   std::size_t total_visible = 0;

   for (auto row : visibility)
      total_visible += std::count_if(row.begin(), row.end(), [](bool b){ return b; });

   std::cout << "Part 1: " << total_visible << std::endl;
   std::cout << "Part 2: " << scenic_score(forest, visibility) << std::endl;
   return 0;
}
