#include <algorithm>
#include <array>
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

enum class PipeDirection : char
{
   Null = 0,
   Start,
   North,
   South,
   West,
   East
};

using Coordinate = std::pair<std::size_t,std::size_t>;

struct Pipe
{
   Coordinate coord;
   std::map<PipeDirection,std::shared_ptr<Pipe>> connectors;
   
   Pipe(char shape)
   {
      switch(shape)
      {
      case '|':
         this->connectors[PipeDirection::North] = nullptr;
         this->connectors[PipeDirection::South] = nullptr;
         break;

      case '-':
         this->connectors[PipeDirection::West] = nullptr;
         this->connectors[PipeDirection::East] = nullptr;
         break;

      case 'L':
         this->connectors[PipeDirection::North] = nullptr;
         this->connectors[PipeDirection::East] = nullptr;
         break;

      case 'J':
         this->connectors[PipeDirection::North] = nullptr;
         this->connectors[PipeDirection::West] = nullptr;
         break;

      case '7':
         this->connectors[PipeDirection::West] = nullptr;
         this->connectors[PipeDirection::South] = nullptr;
         break;

      case 'F':
         this->connectors[PipeDirection::East] = nullptr;
         this->connectors[PipeDirection::South] = nullptr;
         break;

      case 'S':
         this->connectors[PipeDirection::Start] = nullptr;
         break;

      case '.':
      default:
         this->connectors[PipeDirection::Null] = nullptr;
         break;
      }
   }

   char ascii(void) {
      if (this->connectors.find(PipeDirection::Start) != this->connectors.end())
         return 'S';
      else if (this->connectors.find(PipeDirection::North) != this->connectors.end() && this->connectors.find(PipeDirection::South) != this->connectors.end())
         return '|';
      else if (this->connectors.find(PipeDirection::West) != this->connectors.end() && this->connectors.find(PipeDirection::East) != this->connectors.end())
         return '-';
      else if (this->connectors.find(PipeDirection::North) != this->connectors.end() && this->connectors.find(PipeDirection::East) != this->connectors.end())
         return 'L';
      else if (this->connectors.find(PipeDirection::North) != this->connectors.end() && this->connectors.find(PipeDirection::West) != this->connectors.end())
         return 'J';
      else if (this->connectors.find(PipeDirection::South) != this->connectors.end() && this->connectors.find(PipeDirection::West) != this->connectors.end())
         return '7';
      else if (this->connectors.find(PipeDirection::South) != this->connectors.end() && this->connectors.find(PipeDirection::East) != this->connectors.end())
         return 'F';
      else
         return '.';
   }
};

using PipeNetwork = std::map<Coordinate,std::shared_ptr<Pipe>>;

std::pair<std::shared_ptr<Pipe>,PipeNetwork> read_input(std::istream &stream)
{
   std::string line;
   std::vector<std::string> sketch;
   PipeNetwork result;
   std::shared_ptr<Pipe> start;
   std::map<PipeDirection,PipeDirection> opposites = {
      {PipeDirection::West,PipeDirection::East},
      {PipeDirection::East,PipeDirection::West},
      {PipeDirection::North,PipeDirection::South},
      {PipeDirection::South,PipeDirection::North}
   };
      
   while (std::getline(stream, line))
      sketch.push_back(line);

   for (std::size_t y=0; y<sketch.size(); ++y)
   {
      for (std::size_t x=0; x<sketch[0].size(); ++x)
      {
         auto new_pipe = std::make_shared<Pipe>(sketch[y][x]);
         new_pipe->coord = {x,y};
         result[new_pipe->coord] = new_pipe;

         if (new_pipe->connectors.find(PipeDirection::Null) != new_pipe->connectors.end())
            continue;
         else if (new_pipe->connectors.find(PipeDirection::Start) != new_pipe->connectors.end())
         {
            start = new_pipe;
            continue;
         }

         if (x > 0)
         {
            Coordinate prev_x = {x-1,y};

            if (result.find(prev_x) != result.end() && 
                result[prev_x]->connectors.find(PipeDirection::East) != result[prev_x]->connectors.end() &&
                new_pipe->connectors.find(PipeDirection::West) != new_pipe->connectors.end())
            {
               new_pipe->connectors[PipeDirection::West] = result[prev_x];
               result[prev_x]->connectors[PipeDirection::East] = new_pipe;
            }
         }

         if (y > 0)
         {
            Coordinate prev_y = {x,y-1};

            if (result.find(prev_y) != result.end() && 
                result[prev_y]->connectors.find(PipeDirection::South) != result[prev_y]->connectors.end() &&
                new_pipe->connectors.find(PipeDirection::North) != new_pipe->connectors.end())
            {
               new_pipe->connectors[PipeDirection::North] = result[prev_y];
               result[prev_y]->connectors[PipeDirection::South] = new_pipe;
            }
         }
      }
   }
         
   auto start_coord = start->coord;
   std::optional<Coordinate> north_coord, south_coord, east_coord, west_coord;
   std::shared_ptr<Pipe> north_pipe, south_pipe, east_pipe, west_pipe;

   std::set<std::pair<PipeDirection,std::shared_ptr<Pipe>>> possible_connectors;

   south_coord = {start_coord.first,start_coord.second+1};
   east_coord = {start_coord.first+1,start_coord.second};

   if (start_coord.first > 0)
      west_coord = {start_coord.first-1,start_coord.second};
   if (start_coord.second > 0)
      north_coord = {start_coord.first,start_coord.second-1};

   if (north_coord.has_value() && result.find(*north_coord) != result.end())
      north_pipe = result[*north_coord];
   if (south_coord.has_value() && result.find(*south_coord) != result.end())
      south_pipe = result[*south_coord];
   if (west_coord.has_value() && result.find(*west_coord) != result.end())
      west_pipe = result[*west_coord];
   if (east_coord.has_value() && result.find(*east_coord) != result.end())
      east_pipe = result[*east_coord];

   if (north_pipe != nullptr && north_pipe->connectors.find(PipeDirection::South) != north_pipe->connectors.end())
      possible_connectors.insert({PipeDirection::North,north_pipe});
   if (south_pipe != nullptr && south_pipe->connectors.find(PipeDirection::North) != south_pipe->connectors.end())
      possible_connectors.insert({PipeDirection::South,south_pipe});
   if (west_pipe != nullptr && west_pipe->connectors.find(PipeDirection::East) != west_pipe->connectors.end())
      possible_connectors.insert({PipeDirection::West,west_pipe});
   if (east_pipe != nullptr && east_pipe->connectors.find(PipeDirection::West) != east_pipe->connectors.end())
      possible_connectors.insert({PipeDirection::East,east_pipe});

   for (auto possible : possible_connectors)
   {
      start->connectors[possible.first] = possible.second;
      possible.second->connectors[opposites[possible.first]] = start;
   }
      
   start->connectors.erase(start->connectors.find(PipeDirection::Start));
   
   return {start,result};
}

std::pair<std::size_t,std::set<std::shared_ptr<Pipe>>> traverse_pipes(std::shared_ptr<Pipe> start, const PipeNetwork &network)
{
   std::set<std::shared_ptr<Pipe>> visited;
   std::vector<std::pair<std::size_t,std::pair<PipeDirection,std::shared_ptr<Pipe>>>> visiting;
   std::map<PipeDirection,PipeDirection> opposites = {
      {PipeDirection::West,PipeDirection::East},
      {PipeDirection::East,PipeDirection::West},
      {PipeDirection::North,PipeDirection::South},
      {PipeDirection::South,PipeDirection::North}
   };
   std::size_t highest_cost = 0;
   
   visited.insert(start);

   for (auto connector : start->connectors)
      visiting.push_back({1,connector});

   while (visiting.size() > 0)
   {
      auto visit = visiting.front();
      visiting.erase(visiting.begin());

      auto cost = visit.first;
      auto pair = visit.second;
      auto ignore = opposites[pair.first];
      std::optional<std::pair<PipeDirection,std::shared_ptr<Pipe>>> next;

      if (cost > highest_cost)
         highest_cost = cost;
      
      visited.insert(pair.second);
      
      for (auto direction : pair.second->connectors)
      {
         if (direction.first == ignore || visited.find(direction.second) != visited.end())
            continue;

         next = direction;
      }

      if (next.has_value())
         visiting.push_back({cost+1,*next});
   }

   return {highest_cost,visited};
}

struct ZoomedPipe
{
   std::vector<std::vector<std::shared_ptr<Pipe>>> zoom;

   ZoomedPipe() {}
   ZoomedPipe(const ZoomedPipe &other) : zoom(other.zoom) {}
   ZoomedPipe(std::shared_ptr<Pipe> pipe)
   {
      switch (pipe->ascii())
      {
      case '.':
         this->zoom = {{pipe, pipe, pipe, pipe},
                       {pipe, pipe, pipe, pipe},
                       {pipe, pipe, pipe, pipe},
                       {pipe, pipe, pipe, pipe}};
         
         break;

      case '|':
         this->zoom = {{nullptr, pipe, pipe, nullptr},
                       {nullptr, pipe, pipe, nullptr},
                       {nullptr, pipe, pipe, nullptr},
                       {nullptr, pipe, pipe, nullptr}};
         break;

      case '-':
         this->zoom = {{nullptr, nullptr, nullptr, nullptr},
                       {pipe, pipe, pipe, pipe},
                       {pipe, pipe, pipe, pipe},
                       {nullptr, nullptr, nullptr, nullptr}};

         break;

      case 'L':
         this->zoom = {{nullptr, pipe, pipe, nullptr},
                       {nullptr, pipe, pipe, pipe},
                       {nullptr, pipe, pipe, pipe},
                       {nullptr, nullptr, nullptr, nullptr}};
         break;

      case 'J':
         this->zoom = {{nullptr, pipe, pipe, nullptr},
                       {pipe, pipe, pipe, nullptr},
                       {pipe, pipe, pipe, nullptr},
                       {nullptr, nullptr, nullptr, nullptr}};
         break;

      case '7':
         this->zoom = {{nullptr, nullptr, nullptr, nullptr},
                       {pipe, pipe, pipe, nullptr},
                       {pipe, pipe, pipe, nullptr},
                       {nullptr, pipe, pipe, nullptr}};
         break;

      case 'F':
         this->zoom = {{nullptr, nullptr, nullptr, nullptr},
                       {nullptr, pipe, pipe, pipe},
                       {nullptr, pipe, pipe, pipe},
                       {nullptr, pipe, pipe, nullptr}};
         break;
      }
   }

   std::optional<std::shared_ptr<Pipe>> at(const Coordinate &coord) {
      if (coord.first >= 4 || coord.second >= 4)
         return std::nullopt;

      return this->zoom[coord.second][coord.first];
   }
};

std::size_t count_enclosed(std::shared_ptr<Pipe> start, const PipeNetwork &network)
{
   auto loop_data = traverse_pipes(start, network);
   auto loop = loop_data.second;
   std::map<Coordinate,ZoomedPipe> zoomed_network;
   std::map<std::pair<Coordinate,Coordinate>,std::shared_ptr<Pipe>> zoomed_tiles;
   std::set<Coordinate> network_coords;
   std::set<std::pair<Coordinate,Coordinate>> zoomed_loop;

   for (auto pair : network)
   {
      zoomed_network[pair.first] = ZoomedPipe(pair.second);
      network_coords.insert(pair.first);

      for (std::size_t y=0; y<4; ++y)
         for (std::size_t x=0; x<4; ++x)
            zoomed_tiles[{pair.first,{x,y}}] = zoomed_network[pair.first].zoom[y][x];
   }

   std::set<std::pair<Coordinate,Coordinate>> visited;
   std::set<std::shared_ptr<Pipe>> looped_tiles;

   auto max_coord = *network_coords.rbegin();

   for (std::size_t y=0; y<=max_coord.second*4; ++y)
   {
      for (std::size_t x=0; x<=max_coord.first*4; ++x)
      {
         Coordinate outer = {x/4,y/4};
         Coordinate inner = {x%4,y%4};
         std::pair<Coordinate,Coordinate> combined = {outer,inner};

         if (visited.find(combined) != visited.end())
            continue;

         auto pipe = zoomed_tiles[combined];

         if (loop.find(pipe) != loop.end())
            continue;

         std::vector<std::pair<Coordinate,Coordinate>> visiting = {combined};
         std::set<std::shared_ptr<Pipe>> local_visited;
         std::set<std::shared_ptr<Pipe>> borders;

         while (visiting.size() > 0)
         {
            auto visit = visiting.front();
            visiting.erase(visiting.begin());
            
            if (visited.find(visit) != visited.end())
               continue;
            
            visited.insert(visit);

            if (zoomed_tiles.find(visit) == zoomed_tiles.end())
            {
               borders.insert(nullptr);
               continue;
            }

            auto visit_outer = visit.first;
            auto visit_inner = visit.second;
            auto visit_x = visit_outer.first * 4 + visit_inner.first;
            auto visit_y = visit_outer.second * 4 + visit_inner.second;
            auto visit_pipe = zoomed_tiles[visit];

            if (loop.find(visit_pipe) != loop.end())
            {
               borders.insert(visit_pipe);
               continue;
            }
            
            if (visit_pipe != nullptr)
               local_visited.insert(visit_pipe);

            Coordinate north_pair = {visit_x,visit_y-1};
            Coordinate south_pair = {visit_x,visit_y+1};
            Coordinate west_pair = {visit_x-1,visit_y};
            Coordinate east_pair = {visit_x+1,visit_y};

            std::pair<Coordinate,Coordinate> north_coord = {{north_pair.first/4,north_pair.second/4},{north_pair.first%4,north_pair.second%4}};
            std::pair<Coordinate,Coordinate> south_coord = {{south_pair.first/4,south_pair.second/4},{south_pair.first%4,south_pair.second%4}};
            std::pair<Coordinate,Coordinate> west_coord = {{west_pair.first/4,west_pair.second/4},{west_pair.first%4,west_pair.second%4}};
            std::pair<Coordinate,Coordinate> east_coord = {{east_pair.first/4,east_pair.second/4},{east_pair.first%4,east_pair.second%4}};

            if (zoomed_tiles.find(north_coord) == zoomed_tiles.end()) { borders.insert(nullptr); }
            else if (loop.find(zoomed_tiles[north_coord]) != loop.end()) { borders.insert(zoomed_tiles[north_coord]); }
            else if (visited.find(north_coord) == visited.end()) { visiting.push_back(north_coord); }

            if (zoomed_tiles.find(south_coord) == zoomed_tiles.end()) { borders.insert(nullptr); }
            else if (loop.find(zoomed_tiles[south_coord]) != loop.end()) { borders.insert(zoomed_tiles[south_coord]); }
            else if (visited.find(south_coord) == visited.end()) { visiting.push_back(south_coord); }

            if (zoomed_tiles.find(west_coord) == zoomed_tiles.end()) { borders.insert(nullptr); }
            else if (loop.find(zoomed_tiles[west_coord]) != loop.end()) { borders.insert(zoomed_tiles[west_coord]); }
            else if (visited.find(west_coord) == visited.end()) { visiting.push_back(west_coord); }

            if (zoomed_tiles.find(east_coord) == zoomed_tiles.end()) { borders.insert(nullptr); }
            else if (loop.find(zoomed_tiles[east_coord]) != loop.end()) { borders.insert(zoomed_tiles[east_coord]); }
            else if (visited.find(east_coord) == visited.end()) { visiting.push_back(east_coord); }
         }

         if (borders.find(nullptr) != borders.end())
            continue;

         for (auto tile : local_visited)
         {
            looped_tiles.insert(tile);
         }
      }
   }

   return looped_tiles.size();
}

int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);

   auto result = traverse_pipes(puzzle_input.first, puzzle_input.second);
   std::cout << "part 1: " << result.first << std::endl;
   std::cout << "part 2: " << count_enclosed(puzzle_input.first, puzzle_input.second) << std::endl;
   
   return 0;
}
