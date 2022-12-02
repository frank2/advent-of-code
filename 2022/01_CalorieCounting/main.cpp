#include <algorithm>
#include <cstdint>
#include <cstddef>
#include <iostream>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

struct Elf
{
   struct Compare
   {
      bool operator()(const Elf &left, const Elf &right) {
         auto sum_left = left.sum();
         auto sum_right = right.sum();

         if (sum_left == sum_right) { return false; }

         return sum_left < sum_right;
      }
   };
   
   std::vector<std::size_t> calories;
         
   static Elf parse() {
      Elf result;

      while (!std::cin.eof())
      {
         std::string line;
         std::getline(std::cin, line);
         if (line.size() == 0) { break; }

         std::istringstream stream;
         stream.str(line);
         
         std::size_t value;
         stream >> value;
         
         result.calories.push_back(value);
      }

      return result;
   }

   std::size_t sum() const { return std::accumulate(calories.begin(), calories.end(), std::size_t(0)); }
};

std::vector<Elf> read_input() {
   std::vector<Elf> result;

   while (!std::cin.eof()) { result.push_back(Elf::parse()); }

   return result;
}

int main(int argc, char *argv[])
{
   auto elves = read_input();
   std::sort(elves.begin(), elves.end(), Elf::Compare());
   auto most_calories = elves.back();

   std::cout << "Part 1: " << most_calories.sum() << std::endl;

   std::size_t top_3 = 0;
   int i=0;
   auto iter = elves.rbegin();

   for (i,iter; i<3; ++i,++iter)
      top_3 += iter->sum();

   std::cout << "Part 2: " << top_3 << std::endl;
   
   return 0;
}
