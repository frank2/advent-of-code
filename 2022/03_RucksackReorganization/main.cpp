#include <algorithm>
#include <iostream>
#include <set>
#include <string>
#include <vector>

struct Item
{
   struct Compare {
      bool operator() (const Item &left, const Item &right) const {
         if (left.label == right.label) { return false; }

         return left.label < right.label;
      }
   };
   
   char label;

   Item() : label(0) {}
   Item(char label) : label(label) {}
   Item(const Item &other) : label(other.label) {}

   int priority() const {
      if (this->label >= 'a' && this->label <= 'z')
         return static_cast<int>(this->label - 'a' + 1);
      else
         return static_cast<int>(this->label - 'A' + 27);
   }
};

struct Rucksack
{
   std::set<Item, Item::Compare> left, right, joined;

   Rucksack() {}
   Rucksack(std::string rucksack) {
      auto split_point = rucksack.size() / 2;

      for (std::size_t i=0; i<split_point; ++i)
         this->left.insert(rucksack[i]);

      for (std::size_t i=split_point; i<rucksack.size(); ++i)
         this->right.insert(rucksack[i]);

      std::set_union(this->left.begin(), this->left.end(),
                     this->right.begin(), this->right.end(),
                     std::inserter(this->joined, this->joined.end()),
                     Item::Compare());
   }
   Rucksack(const Rucksack &other) : left(other.left), right(other.right), joined(other.joined) {}

   Item overlap() const {
      std::set<Item, Item::Compare> result;
      std::set_intersection(this->left.begin(), this->left.end(),
                            this->right.begin(), this->right.end(),
                            std::inserter(result, result.end()),
                            Item::Compare());
      
      return *result.begin();
   }
};

struct Group
{
   Rucksack first, second, third;

   Group() {}
   Group(Rucksack &first, Rucksack &second, Rucksack &third) : first(first), second(second), third(third) {}
   Group(const Group &other) : first(other.first), second(other.second), third(other.third) {}

   Item badge() {
      std::set<Item, Item::Compare> result;
      std::vector<Rucksack *> group = { &this->second, &this->third };

      result = this->first.joined;

      for (auto member : group)
      {
         std::set<Item, Item::Compare> intersection;
         std::set_intersection(result.begin(), result.end(),
                               member->joined.begin(), member->joined.end(),
                               std::inserter(intersection, intersection.end()),
                               Item::Compare());

         result = intersection;
      }

      return *result.begin();
   }
};

std::vector<Rucksack> parse() {
   std::vector<Rucksack> result;
   
   while (!std::cin.eof())
   {      
      std::string line;
      std::getline(std::cin, line);
      if (line.size() == 0) { break; }
      result.push_back(Rucksack(line));
   }

   return result;
}

int main(int argc, char *argv[])
{
   auto sacks = parse();
   std::size_t priority = 0;

   for (auto sack : sacks)
      priority += sack.overlap().priority();

   std::cout << "Part 1: " << priority << std::endl;

   priority = 0;

   for (std::size_t i=0; i<sacks.size(); i+=3)
   {
      auto group = Group(sacks[i], sacks[i+1], sacks[i+2]);
      priority += group.badge().priority();
   }

   std::cout << "Part 2: " << priority << std::endl;

   return 0;
}
