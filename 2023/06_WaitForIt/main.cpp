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

using Races = std::pair<std::vector<std::size_t>,std::vector<std::size_t>>;

Races read_input(std::istream &stream)
{
   Races result;
   std::string time_line, distance_line;
   
   std::getline(stream, time_line);
   std::stringstream time_stream(time_line.substr(time_line.find(": ")+2));

   while (!time_stream.eof())
   {
      std::size_t time;
      time_stream >> time;
      result.first.push_back(time);
   }
                                                  
   std::getline(stream, distance_line);
   std::stringstream distance_stream(distance_line.substr(distance_line.find(": ")+2));

   while (!distance_stream.eof())
   {
      std::size_t distance;
      distance_stream >> distance;
      result.second.push_back(distance);
   }
      
   return result;
}

Races read_input_another(std::istream &stream)
{
   Races result;
   std::string time_line, distance_line;
   
   std::getline(stream, time_line);
   std::stringstream time_stream(time_line.substr(time_line.find(": ")+2));
   std::string kern_time;

   while (!time_stream.eof())
   {
      std::string time;
      time_stream >> time;
      kern_time += time;
   }
                                                  
   std::getline(stream, distance_line);
   std::stringstream distance_stream(distance_line.substr(distance_line.find(": ")+2));
   std::string kern_distance;

   while (!distance_stream.eof())
   {
      std::string distance;
      distance_stream >> distance;
      kern_distance += distance;
   }

   result.first.push_back(std::stoll(kern_time));
   result.second.push_back(std::stoll(kern_distance));
      
   return result;
}

std::size_t perform_races(const Races &races, std::size_t acceleration)
{
   std::vector<std::vector<std::pair<std::size_t,std::size_t>>> race_results;
   
   for (std::size_t i=0; i<races.first.size(); ++i)
   {
      std::size_t time = races.first[i];
      std::size_t distance = races.second[i];
      std::size_t performance = 0;
      std::vector<std::pair<std::size_t,std::size_t>> winners;

      for (std::size_t button_press=1; button_press<time; ++button_press)
      {
         performance = acceleration * button_press * (time - button_press);
         
         if (performance > distance)
            winners.push_back({button_press,performance});
      }

      race_results.push_back(winners);
   }

   std::size_t result = 1;

   for (auto race_result : race_results)
      result *= race_result.size();

   return result;
}

int main(int argc, char *argv[])
{
   std::string input_data, line;

   while (std::getline(std::cin, line))
      input_data += line + std::string("\n");

   std::stringstream input_stream(input_data);
   std::cout << "part 1: " << perform_races(read_input(input_stream), 1) << std::endl;

   input_stream = std::stringstream(input_data);
   std::cout << "part 2: " << perform_races(read_input_another(input_stream), 1) << std::endl;
   
   return 0;
}
