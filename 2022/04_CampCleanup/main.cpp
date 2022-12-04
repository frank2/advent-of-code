#include <iostream>
#include <optional>
#include <string>
#include <utility>
#include <vector>

struct Section {
   int low, high;

   Section() {}
   Section(int low, int high) : low(low), high(high) {}
   Section(const Section &other) : low(other.low), high(other.high) {}

   static std::optional<Section> parse(std::string section) {
      auto split = section.find("-");
      if (split == std::string::npos) { return std::nullopt; }
      auto left = section.substr(0,split);
      auto right = section.substr(split+1);

      return Section(std::stoi(left), std::stoi(right));
   }

   bool contains(const Section &other) const {
      return other.low >= this->low && other.high <= this->high;
   }

   bool overlaps(const Section &other) const {
      return this->low <= other.high && other.low <= this->high;
   }
};

struct Group
{
   Section first, second;

   Group() {}
   Group(const Section &first, const Section &second) : first(first), second(second) {}
   Group(const Group &other) : first(other.first), second(other.second) {}

   static std::optional<Group> parse() {
      std::string line;
      std::getline(std::cin, line);
      if (line.size() == 0) { return std::nullopt; }

      auto split = line.find(",");
      if (split == std::string::npos) { return std::nullopt; }
      auto left = line.substr(0,split);
      auto right = line.substr(split+1);

      return Group(*Section::parse(left), *Section::parse(right));
   }

   bool contains() const {
      return this->first.contains(this->second) || this->second.contains(this->first);
   }

   bool overlaps() const {
      return this->first.overlaps(this->second) || this->second.overlaps(this->first);
   }
};

int main(int argc, char *argv[])
{
   std::vector<Group> groups;
   std::optional<Group> group = Group::parse();

   while (group.has_value())
   {
      groups.push_back(*group);
      group = Group::parse();
   }

   std::size_t containing = 0;

   for (auto group : groups)
      if (group.contains())
         ++containing;

   std::cout << "Part 1: " << containing << std::endl;
   std::size_t overlapping = 0;

   for (auto group : groups)
      if (group.overlaps())
         ++overlapping;

   std::cout << "Part 2: " << overlapping << std::endl;
   
   return 0;
}
