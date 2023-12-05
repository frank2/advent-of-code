#include <cstdint>
#include <iostream>
#include <map>
#include <memory>
#include <numeric>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

enum class Part : std::size_t
{
   Number,
   Gear
};

using PartSpec = std::pair<Part,std::string>;
using Coordinate = std::pair<std::size_t,std::size_t>;

struct Machine
{
   std::map<Coordinate,std::shared_ptr<PartSpec>> blueprint;
   std::map<Part,std::vector<Coordinate>> parts;

   std::set<std::shared_ptr<PartSpec>> part_numbers(const Coordinate &) const;
   std::size_t sum_part_numbers() const;
   std::size_t gear_ratio() const;
};

std::set<std::shared_ptr<PartSpec>> Machine::part_numbers(const Coordinate &coord) const
{
   std::set<std::shared_ptr<PartSpec>> result;
   
   for (std::intptr_t y=-1; y<=1; ++y)
   {
      for (std::intptr_t x=-1; x<=1; ++x)
      {
         if (x == 0 && y == 0)
            continue;

         std::size_t dx = coord.first + x;
         std::size_t dy = coord.second + y;
         Coordinate dc = std::make_pair(dx,dy);
            
         if (this->blueprint.find(dc) != this->blueprint.end() && this->blueprint.at(dc)->first == Part::Number)
            result.insert(this->blueprint.at(dc));
      }
   }

   return result;
}

std::size_t Machine::sum_part_numbers() const
{
   std::set<std::shared_ptr<PartSpec>> valid_parts;
   std::size_t result = 0;
   
   for (auto coord : this->parts.at(Part::Gear))
      for (auto part : this->part_numbers(coord))
         valid_parts.insert(part);

   for (auto part : valid_parts)
      result += std::stoll(part->second);

   return result;
}

std::size_t Machine::gear_ratio() const
{
   std::size_t result = 0;
   
   for (auto coord : this->parts.at(Part::Gear))
   {
      if (this->blueprint.at(coord)->second != "*")
         continue;

      auto other_numbers = this->part_numbers(coord);

      if (other_numbers.size() != 2)
         continue;

      std::vector vec_set(other_numbers.begin(), other_numbers.end());

      result += std::stoll(vec_set[0]->second) * std::stoll(vec_set[1]->second);
   }

   return result;
}
            
Machine read_input(std::istream &stream)
{
   Machine result;
   result.parts = {{Part::Number, {}}, {Part::Gear, {}}};
   std::size_t y = 0;
   
   for (std::string line; std::getline(stream, line); ++y)
   {
      std::string part_stack;
      std::optional<Part> last_part;

      for (std::size_t x=0; x<line.size(); ++x)
      {
         std::optional<Part> this_part;
         
         if (line[x] >= '0' && line[x] <= '9')
            this_part = Part::Number;         
         else if (line[x] != '.')
            this_part = Part::Gear;
         else
            this_part = std::nullopt;

         if (this_part != last_part && part_stack.size() > 0)
         {
            std::shared_ptr<PartSpec> part = std::make_shared<PartSpec>(std::make_pair(*last_part, part_stack));

            for (std::size_t start=x-part_stack.size(); start<x; ++start)
            {
               result.blueprint[std::make_pair(start,y)] = part;
               result.parts[*last_part].push_back(std::make_pair(start,y));
            }

            part_stack.clear();
         }
         
         if (this_part.has_value())
            part_stack.push_back(line[x]);

         last_part = this_part;
      }

      if (last_part.has_value())
      {
         std::shared_ptr<PartSpec> part = std::make_shared<PartSpec>(std::make_pair(*last_part, part_stack));
         std::size_t x = line.size();

         for (std::size_t start=x-part_stack.size(); start<x; ++start)
         {
            result.blueprint[std::make_pair(start,y)] = part;
            result.parts[*last_part].push_back(std::make_pair(start,y));
         }
      }
   }
      
   return result;
}

int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);

   std::cout << "part 1: " << puzzle_input.sum_part_numbers() << std::endl;
   std::cout << "part 2: " << puzzle_input.gear_ratio() << std::endl;

   return 0;
}
