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

using DeltaVector = std::vector<std::intptr_t>;

std::vector<DeltaVector> read_input(std::istream &stream)
{
   std::string line;
   std::vector<DeltaVector> result;
      
   while (std::getline(stream, line))
   {
      std::stringstream line_stream(line);
      std::intptr_t delta;

      result.push_back(DeltaVector());

      while (!line_stream.eof())
      {
         line_stream >> delta;
         result.back().push_back(delta);
      }
   }
                                                       
   return result;
}

DeltaVector delta_vector(const DeltaVector &input)
{
   DeltaVector result;

   for (std::size_t i=0; i<input.size()-1; ++i)
      result.push_back(input[i+1]-input[i]);

   return result;
}

std::intptr_t next_value(const DeltaVector &input)
{
   std::intptr_t result = 0;
   std::vector<DeltaVector> deltas = { input };
   DeltaVector next_deltas;
   bool all_zeros = false;

   do
   {
      next_deltas = delta_vector(deltas.back());
      
      deltas.push_back(next_deltas);
      all_zeros = true;

      for (auto d : next_deltas)
         if (d != 0)
            all_zeros = false;
   } while (!all_zeros);

   for (std::intptr_t i=deltas.size()-1; i>=0; --i)
      result += deltas[i].back();

   return result;
}

std::intptr_t next_value_another(const DeltaVector &input)
{
   std::intptr_t result = 0;
   std::vector<DeltaVector> deltas = { input };
   DeltaVector next_deltas;
   bool all_zeros = false;

   do
   {
      next_deltas = delta_vector(deltas.back());
      
      deltas.push_back(next_deltas);
      all_zeros = true;

      for (auto d : next_deltas)
         if (d != 0)
            all_zeros = false;
   } while (!all_zeros);

   for (std::intptr_t i=deltas.size()-1; i>=0; --i)
      result = deltas[i].front() - result;

   return result;
}

std::intptr_t sum_next_values(const std::vector<DeltaVector> &input)
{
   std::intptr_t result = 0;

   for (auto deltas : input)
   {
      std::intptr_t next = next_value(deltas);
      result += next;
   }

   return result;
}

std::intptr_t sum_next_values_another(const std::vector<DeltaVector> &input)
{
   std::intptr_t result = 0;

   for (auto deltas : input)
   {
      std::intptr_t next = next_value_another(deltas);
      result += next;
   }

   return result;
}
 
int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);

   std::cout << "part 1: " << sum_next_values(puzzle_input) << std::endl;
   std::cout << "part 2: " << sum_next_values_another(puzzle_input) << std::endl;
   
   return 0;
}
