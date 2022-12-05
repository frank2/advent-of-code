#include <iostream>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

using Stack = std::vector<char>;

struct Supply
{
   std::vector<Stack> supplies;

   Supply() {}
   Supply(const Supply &other) : supplies(other.supplies) {}

   static std::optional<Supply> parse() {
      std::string line;
      std::vector<std::string> line_stack;
      Supply result;

      while (std::getline(std::cin, line) && line.size() > 0)
         line_stack.push_back(line);

      if (line_stack.size() == 0) { return std::nullopt; }
      
      std::string stack_count = line_stack.back();
      line_stack.pop_back();
      
      std::size_t stacks = (stack_count.size()+1)/4;
      if (stacks == 0) { return std::nullopt; }
      
      result.supplies = std::vector<Stack>(stacks);

      while (line_stack.size() > 0)
      {
         line = line_stack.back();
         line_stack.pop_back();

         for (std::size_t i=0; i<stacks; ++i)
         {
            char label = line[1+i*4];
            if (label == ' ') { continue; }

            result.supplies[i].push_back(label);
         }
      }

      return result;
   }

   Supply clone() {
      Supply result = *this;
      return result;
   }
            
   void move9000(int count, int from, int to)
   {
      for (std::size_t i=0; i<count; ++i)
      {
         this->supplies[to].push_back(this->supplies[from].back());
         this->supplies[from].pop_back();
      }
   }

   void move9001(int count, int from, int to)
   {
      Stack stack;
      
      for (std::size_t i=0; i<count; ++i)
      {
         stack.push_back(this->supplies[from].back());
         this->supplies[from].pop_back();
      }

      this->supplies[to].insert(this->supplies[to].end(), stack.rbegin(), stack.rend());
   }

   Stack message(void) const
   {
      Stack result;
      
      for (std::size_t i=0; i<this->supplies.size(); ++i)
      {
         if (this->supplies[i].size() == 0) { continue; }
         result.push_back(this->supplies[i].back());
      }

      return result;
   }
};

struct Arrangement
{
   int count, from, to;

   Arrangement() {}
   Arrangement(const Arrangement &other) : count(other.count), from(other.from), to(other.to) {}

   static std::optional<Arrangement> parse() {
      std::string line;

      if (!std::getline(std::cin, line) || line.size() == 0) { return std::nullopt; }

      std::istringstream stream;
      stream.str(line);
      
      std::string move_s, from_s, to_s;
      Arrangement arrangement;

      stream >> move_s >> arrangement.count
             >> from_s >> arrangement.from
             >> to_s >> arrangement.to;

      --arrangement.from;
      --arrangement.to;
      
      return arrangement;
   }

   void move9000(Supply &supply) {
      supply.move9000(this->count, this->from, this->to);
   }

   void move9001(Supply &supply) {
      supply.move9001(this->count, this->from, this->to);
   }
};

int main(int argc, char *argv[]) {
   auto supply9000 = Supply::parse();

   if (!supply9000.has_value())
   {
      std::cerr << "Failed to parse supply" << std::endl;
      return 1;
   }

   auto supply9001 = supply9000->clone();
   
   std::vector<Arrangement> arrangements;
   auto parsed = Arrangement::parse();
   
   while (parsed.has_value())
   {
      arrangements.push_back(*parsed);
      parsed = Arrangement::parse();
   }

   for (auto arrangement : arrangements)
      arrangement.move9000(*supply9000);
   
   auto msg = supply9000->message();

   std::cout << "Part 1: ";
   
   for (auto c : msg)
      std::cout << c;

   std::cout << std::endl;
   
   for (auto arrangement : arrangements)
      arrangement.move9001(supply9001);
   
   msg = supply9001.message();

   std::cout << "Part 2: ";
   
   for (auto c : msg)
      std::cout << c;

   std::cout << std::endl;

   return 0;
}
