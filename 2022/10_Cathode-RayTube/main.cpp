#include <array>
#include <iostream>
#include <numeric>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

using Instruction = std::pair<std::string, std::optional<int>>;

class Program
{
   std::vector<Instruction> instructions;
   std::array<std::array<char, 40>, 6> screen;
   int reg;

   void init_screen() {
      for (std::size_t y=0; y<this->screen.size(); ++y)
         for (std::size_t x=0; x<this->screen[0].size(); ++x)
            this->screen[y][x] = ' ';
   }

public:
   Program() : reg(1) { this->init_screen(); }
   Program(std::vector<Instruction> instructions) : instructions(instructions), reg(1) { this->init_screen(); }
   Program(const Program &other) : instructions(other.instructions), screen(other.screen), reg(other.reg) {}

   static Program parse() {
      std::vector<Instruction> instructions;
      std::string line;

      while (std::getline(std::cin, line))
      {
         std::istringstream parse;
         parse.str(line);

         std::string insn;
         std::optional<int> arg = std::nullopt;
         parse >> insn;

         if (insn == "addx")
         {
            int value;
            parse >> value;
            arg = value;
         }

         instructions.push_back(std::make_pair(insn, arg));
      }

      return Program(instructions);
   }

   std::vector<int> execute() {
      std::size_t cycles = 0;
      std::size_t current_cycle = 0;
      std::size_t index = 0;
      std::vector<int> result;

      while (index < this->instructions.size())
      {
         auto insn = this->instructions[index];
         auto mnem = insn.first;
         auto arg = insn.second;
         auto screen_index = cycles % 40;
         auto screen_row = cycles / 40;

         ++cycles;

         if (this->reg - 1 == screen_index || this->reg == screen_index || this->reg+1 == screen_index)
            this->screen[screen_row][screen_index] = '#';

         if ((cycles+20) % 40 == 0)
         {
            result.push_back(this->reg * static_cast<int>(cycles));
         }

         ++current_cycle;

         if (mnem == "noop" && current_cycle == 1) { ++index; current_cycle = 0; }
         else if (mnem == "addx" && current_cycle == 2)
         {
            this->reg += *arg;
            current_cycle = 0;
            ++index;
         }
      }

      return result;
   }

   void print_screen() {
      for (std::size_t y=0; y<this->screen.size(); ++y)
      {
         for (std::size_t x=0; x<this->screen[0].size(); ++x)
         {
            std::cout << this->screen[y][x];
         }

         std::cout << std::endl;
      }
   }
};

int main (int argc, char *argv[])
{
   auto program = Program::parse();
   auto result = program.execute();

   std::cout << "Part 1: " << std::accumulate(result.begin(), result.end(), 0) << std::endl;
   std::cout << "Part 2:" << std::endl;
   program.print_screen();
   return 0;
}
