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

enum class Hand : std::uint64_t
{
   HighCard = 0,
   Pair,
   TwoPair,
   ThreeOfAKind,
   FullHouse,
   FourOfAKind,
   FiveOfAKind
};

struct CamelCards
{
   static std::map<char,std::uint64_t> card_values;
   static std::map<char,std::uint64_t> card_values_joker;

   std::string cards;
   std::uint64_t bid;
   bool joker;

   CamelCards() { this->joker = false; }
   CamelCards(const std::string &cards, std::uint64_t bid) : cards(cards), bid(bid) { this->joker = false; }
   CamelCards(const CamelCards &other) : cards(other.cards), bid(other.bid), joker(other.joker) {}
   
   std::pair<Hand,std::map<char,std::size_t>> evaluate() const;
   bool operator<(const CamelCards &other) const;
};

std::map<char,std::uint64_t> CamelCards::card_values = {
   {'2', 2}, {'3', 3}, {'4', 4}, {'5', 5},
   {'6', 6}, {'7', 7}, {'8', 8}, {'9', 9},
   {'T', 10}, {'J', 11}, {'Q', 12}, {'K', 13}, {'A', 14}
};

std::map<char,std::uint64_t> CamelCards::card_values_joker = {
   {'2', 2}, {'3', 3}, {'4', 4}, {'5', 5},
   {'6', 6}, {'7', 7}, {'8', 8}, {'9', 9},
   {'T', 10}, {'J', 0}, {'Q', 12}, {'K', 13}, {'A', 14}
};

std::pair<Hand,std::map<char,std::size_t>> CamelCards::evaluate() const
{
   std::map<char,std::size_t> result;
   Hand hand;

   for (auto c : this->cards)
   {
      if (result.find(c) == result.end())
         result[c] = 1;
      else
         result[c]++;
   }
   
   if (this->joker && result.find('J') != result.end())
   {
      std::optional<char> highest_count, highest_card;
      
      for (auto c : this->cards)
      {
         if (c == 'J')
            continue;

         if (!highest_card.has_value() || this->card_values[c] > this->card_values[*highest_card])
            highest_card = c;

         if (!highest_count.has_value() || result[c] > result[*highest_count])
            highest_count = c;
      }

      if (result.size() == 5)
         result[*highest_card] += result['J'];
      else if (result.size() == 3)
      {
         highest_card = std::nullopt;

         for (auto pair : result)
         {
            if (pair.first == 'J')
               continue;
            
            if (!highest_card.has_value() || result[pair.first] > result[*highest_card])
               highest_card = pair.first;
         }

         result[*highest_card] += result['J'];
      }
      else
         result[*highest_count] += result['J'];
      
      result.erase(result.find('J'));
   }

   for (auto count : result)
   {
      switch(count.second)
      {
      case 5:
         return {Hand::FiveOfAKind,result};
      case 4:
         return {Hand::FourOfAKind,result};
      case 3:
      {
         if (result.size() == 2)
            return {Hand::FullHouse,result};
         else
            return {Hand::ThreeOfAKind,result};
      }
      case 2:
      {
         if (result.size() == 3)
            return {Hand::TwoPair,result};
         else if (result.size() == 2)
            return {Hand::FullHouse,result};
         else
            return {Hand::Pair,result};
      }
      }
   }

   return {Hand::HighCard,result};
}
      
bool CamelCards::operator<(const CamelCards &other) const
{
   auto left_eval = this->evaluate();
   auto right_eval = other.evaluate();

   if (left_eval.first != right_eval.first)
      return left_eval.first < right_eval.first;

   for (std::size_t i=0; i<this->cards.size(); ++i)
   {
      if (this->cards[i] == other.cards[i])
         continue;

      if (this->joker)
         return this->card_values_joker[this->cards[i]] < this->card_values_joker[other.cards[i]];
      else
         return this->card_values[this->cards[i]] < this->card_values[other.cards[i]];
   }

   return false;
}
   
std::vector<CamelCards> read_input(std::istream &stream)
{
   std::vector<CamelCards> result;
   std::string line;
      
   while (std::getline(stream, line))
   {
      std::string cards, bid;
      std::stringstream line_stream(line);
      line_stream >> cards >> bid;

      result.push_back({cards, std::stoull(bid)});
   }
                                                       
   return result;
}

std::uint64_t rank(const std::vector<CamelCards> &cards)
{
   std::uint64_t result = 0;

   for (std::size_t i=0; i<cards.size(); ++i)
   {
      auto eval = cards[i].evaluate();
      std::cout << cards[i].cards << ": " << (std::uint64_t)eval.first << std::endl;
      
      result += cards[i].bid * (i+1);
   }

   return result;
}

int main(int argc, char *argv[])
{
   auto puzzle_input = read_input(std::cin);
   std::sort(puzzle_input.begin(), puzzle_input.end());

   std::cout << "part 1: " << rank(puzzle_input) << std::endl;

   for (auto &cards : puzzle_input)
      cards.joker = true;

   std::sort(puzzle_input.begin(), puzzle_input.end());

   std::cout << "part 2: " << rank(puzzle_input) << std::endl;
   
   return 0;
}
