#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <set>
#include <string>
#include <vector>

enum Direction
{
   LEFT = 0,
   UP,
   RIGHT,
   DOWN,
   MAX
};

struct Coordinate
{
   struct Compare
   {
      bool operator()(const Coordinate &left, const Coordinate &right) const {
         if (left.x == right.x && left.y == right.y) { return false; }

         if (left.x != right.x)
            return left.x < right.x;

         return left.y < right.y;
      }
   };
   
   std::int32_t x, y;

   Coordinate() : x(0), y(0) {}
   Coordinate(std::int32_t x, std::int32_t y) : x(x), y(y) {}
   Coordinate(const Coordinate &other) : x(other.x), y(other.y) {}

   Coordinate distance(const Coordinate &other) {
      return Coordinate(other.x - this->x, other.y - this->y);
   }

   Coordinate move(const Direction &dir, std::int32_t distance)
   {
      Coordinate result = *this;
      
      switch (dir)
      {
      case Direction::LEFT:
         result.x -= distance;
         break;

      case Direction::UP:
         result.y += distance;
         break;

      case Direction::RIGHT:
         result.x += distance;
         break;

      case Direction::DOWN:
         result.y -= distance;
         break;
      }

      return result;
   }
};

using CoordSet = std::set<Coordinate, Coordinate::Compare>;

using Velocity = std::pair<Direction, std::int32_t>;
 
class Rope
{
private:
   std::vector<Coordinate> knots;
   
public:
   Rope(std::size_t knots=2) : knots(std::vector<Coordinate>(knots)) {}
   Rope(const Rope &other) : knots(other.knots) {}

   CoordSet move(const Direction &dir, std::int32_t distance)
   {
      CoordSet result = { this->knots.back() };
      
      for (std::int32_t i=0; i<distance; ++i)
      {
         this->knots[0] = this->knots[0].move(dir, 1);

         for (std::size_t j=1; j<this->knots.size(); ++j)
         {
            auto prev_knot = this->knots[j-1];
            auto distance = this->knots[j].distance(prev_knot);

            if (std::abs(distance.x) > 1 || std::abs(distance.y) > 1)
            {
               if (std::abs(distance.x) > 1 && std::abs(distance.y) > 1)
               {
                  this->knots[j].x += distance.x + ((distance.x < 0) ? 1 : -1);
                  this->knots[j].y += distance.y + ((distance.y < 0) ? 1 : -1);
               }
               else if (std::abs(distance.x) > 1)
               {
                  this->knots[j].x += distance.x + ((distance.x < 0) ? 1 : -1);

                  if (std::abs(distance.y) > 0)
                     this->knots[j].y += distance.y;
               }
               else if (std::abs(distance.y) > 1)
               {
                  this->knots[j].y += distance.y + ((distance.y < 0) ? 1 : -1);

                  if (std::abs(distance.x) > 0)
                     this->knots[j].x += distance.x;
               }
            }
         }

         result.insert(this->knots.back());
      }

      return result;
   }
};

std::vector<Velocity> parse_simulation() {
   char direction;
   std::int32_t distance;
   std::string line;
   std::vector<Velocity> result;

   while (std::getline(std::cin, line))
   {
      std::istringstream parse;
      parse.str(line);
      parse >> direction >> distance;

      Direction dir;

      if (direction == 'L')
         dir = Direction::LEFT;
      else if (direction == 'U')
         dir = Direction::UP;
      else if (direction == 'R')
         dir = Direction::RIGHT;
      else if (direction == 'D')
         dir = Direction::DOWN;
      
      result.push_back(std::make_pair(dir, distance));
   }

   return result;
}

int main(int argc, char *argv[])
{
   Rope p1;
   CoordSet visits;
   auto sim = parse_simulation();

   for (auto v : sim)
      for (auto coord : p1.move(v.first, v.second))
         visits.insert(coord);

   std::cout << "Part 1: " << visits.size() << std::endl;

   Rope p2(10);
   visits = CoordSet();

   for (auto v : sim)
      for (auto coord : p2.move(v.first, v.second))
         visits.insert(coord);

   std::cout << "Part 2: " << visits.size() << std::endl;
   
   return 0;
}
