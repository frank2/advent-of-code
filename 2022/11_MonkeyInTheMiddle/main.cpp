#include <algorithm>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

struct Monkey;

using Monkeys = std::vector<Monkey>;

struct Monkey
{
   std::vector<std::uint64_t> items;
   std::pair<std::optional<std::uint64_t>, char> operation;
   std::pair<std::uint64_t, std::pair<std::size_t, std::size_t>> test;

   Monkey() {}
   Monkey(const Monkey &other) : items(other.items), operation(other.operation), test(other.test) {}

   static std::optional<Monkey> parse() {
      std::string line;
      if (!std::getline(std::cin, line)) { return std::nullopt; }
      if (line.find("Monkey") == std::string::npos) { return std::nullopt; }

      Monkey result;
      if (!std::getline(std::cin, line)) { return std::nullopt; }
      if (line.find("  Starting items: ") == std::string::npos) { return std::nullopt; }

      std::istringstream item_parse;
      item_parse.str(line.substr(std::strlen("  Starting items: ")));

      while (item_parse.tellg() != -1)
      {
         std::string item;
         item_parse >> item;

         if (item.find(",") != std::string::npos)
            item = item.substr(0,item.find(","));

         result.items.push_back(std::stoi(item));
      }

      if (!std::getline(std::cin, line)) { return std::nullopt; }
      if (line.find("  Operation: new = old ") == std::string::npos) { return std::nullopt; }

      std::istringstream operation_parse;
      operation_parse.str(line.substr(std::strlen("  Operation: new = old ")));
      std::string op, value;
      operation_parse >> op >> value;

      if (value == "old")
         result.operation = std::make_pair(std::nullopt, op[0]);
      else
         result.operation = std::make_pair(std::stoi(value), op[0]);

      if (!std::getline(std::cin, line)) { return std::nullopt; }
      if (line.find("  Test: divisible by ") == std::string::npos) { return std::nullopt; }

      int divisor = std::stoi(line.substr(std::strlen("  Test: divisible by ")));

      if (!std::getline(std::cin, line)) { return std::nullopt; }
      if (line.find("    If true: throw to monkey ") == std::string::npos) { return std::nullopt; }

      int true_monkey = std::stoi(line.substr(std::strlen("    If true: throw to monkey ")));

      if (!std::getline(std::cin, line)) { return std::nullopt; }
      if (line.find("    If false: throw to monkey ") == std::string::npos) { return std::nullopt; }

      int false_monkey = std::stoi(line.substr(std::strlen("    If false: throw to monkey ")));

      result.test = std::make_pair(divisor, std::make_pair(true_monkey, false_monkey));

      if (!std::getline(std::cin, line) || line.size() == 0)
         return result;
      else
         return std::nullopt;
   }

   int inspect(Monkeys &monkeys, std::optional<std::uint64_t> lcm) {
      int inspections = 0;
      
      while (this->items.size() > 0) {
         auto item = this->items.front();
         this->items.erase(this->items.begin());

         std::uint64_t target_value;

         if (this->operation.first.has_value())
            target_value = *this->operation.first;
         else
            target_value = item;

         std::uint64_t result;

         if (this->operation.second == '+')
            result = item + target_value;
         else if (this->operation.second == '*')
            result = item * target_value;

         if (lcm.has_value())
            result %= *lcm;
         else
            result /= 3;

         bool test = (result % this->test.first) == 0;

         if (test)
            monkeys[this->test.second.first].items.push_back(result);
         else
            monkeys[this->test.second.second].items.push_back(result);

         ++inspections;
      }

      return inspections;
   }
};

int main(int argc, char *argv[])
{
   Monkeys monkeys_p1;
   
   auto monkey = Monkey::parse();

   while (monkey.has_value())
   {
      monkeys_p1.push_back(*monkey);
      monkey = Monkey::parse();
   }

   auto monkeys_p2 = monkeys_p1;
   auto inspections = std::vector<std::uint64_t>(monkeys_p1.size(), 0);

   for (std::size_t i=0; i<20; ++i)
      for (std::size_t j=0; j<monkeys_p1.size(); ++j)
         inspections[j] += monkeys_p1[j].inspect(monkeys_p1, std::nullopt);
   
   std::sort(inspections.begin(), inspections.end());
   std::reverse(inspections.begin(), inspections.end());
   std::cout << "Part 1: " << inspections[0] * inspections[1] << std::endl;

   std::uint64_t lcm = 1;

   for (auto monkey : monkeys_p2)
      lcm *= monkey.test.first;

   inspections = std::vector<std::uint64_t>(monkeys_p2.size(), 0);

   for (std::size_t i=0; i<10000; ++i)
      for (std::size_t j=0; j<monkeys_p2.size(); ++j)
         inspections[j] += monkeys_p2[j].inspect(monkeys_p2, lcm);
      
   std::sort(inspections.begin(), inspections.end());
   std::reverse(inspections.begin(), inspections.end());
   std::cout << "Part 2: " << inspections[0] * inspections[1] << std::endl;

   return 0;
}
