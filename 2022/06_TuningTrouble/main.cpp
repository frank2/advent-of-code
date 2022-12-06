#include <iostream>
#include <iterator>
#include <optional>
#include <set>
#include <string>

std::optional<std::string::iterator> find_marker(std::string &str, std::size_t size)
{
   if (str.size() < size) { return str.end(); }

   auto start = str.begin();
   auto end = std::next(start, size);

   while (1)
   {
      std::set unique(start, end);

      if (unique.size() == size) { return end; }
      if (end == str.end()) { return std::nullopt; }
      
      ++start; ++end;
   }
}

int main(int argc, char *argv[])
{
   std::string packet;
   if (!std::getline(std::cin, packet)) { return 1; }

   auto marker = find_marker(packet, 4);
   if (!marker.has_value()) { return 2; }
   
   std::cout << "Part 1: " << *marker - packet.begin() << std::endl;

   auto message = find_marker(packet, 14);
   if (!message.has_value()) { return 3; }
   
   std::cout << "Part 2: " << *message - packet.begin() << std::endl;
   return 0;
}
