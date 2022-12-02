#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <utility>

enum Shape {
   ROCK = 0,
   PAPER,
   SCISSORS
};

enum Result {
   LOSS = 0,
   DRAW,
   WIN
};

using Match = std::pair<Shape,Shape>;

std::vector<Match> parse()
{
   std::vector<Match> result;
   
   while (!std::cin.eof()) {
      std::string line;
      std::getline(std::cin, line);
      if (line.size() == 0) { break; }

      std::istringstream stream;
      stream.str(line);
      
      char opponent, play;
      stream >> opponent;
      stream >> play;

      result.push_back(std::make_pair(static_cast<Shape>(opponent - 'A'), static_cast<Shape>(play - 'X')));
   }

   return result;
}

int main(int argc, char *argv[]) {
   const Result match_table[3][3] = {
      // vs. Rock
      { Result::DRAW, Result::WIN, Result::LOSS },
      // vs. Paper
      { Result::LOSS, Result::DRAW, Result::WIN },
      // vs. Scissors
      { Result::WIN, Result::LOSS, Result::DRAW }
   };
   auto strategy = parse();
   int score = 0;

   for (auto match : strategy)
   {
      auto result = match_table[match.first][match.second];
                                                                              
      score += static_cast<int>(match.second)+1;
      score += static_cast<int>(result)*3;
   }

   std::cout << "Part 1: " << score << std::endl;
   score = 0;

   for (auto match : strategy)
   {
      auto target_result = static_cast<Result>(match.second);
      Shape target_shape;

      for (int i=0; i<3; ++i)
      {
         if (MATCH_TABLE[match.first][i] == target_result)
         {
            target_shape = static_cast<Shape>(i);
            break;
         }
      }

      score += static_cast<int>(target_shape)+1;
      score += static_cast<int>(target_result)*3;
   }

   std::cout << "Part 2: " << score << std::endl;

   return 0;
}
