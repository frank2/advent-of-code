#include <iostream>
#include <limits>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <vector>

struct File
{
   std::string filename;
   std::size_t size;

   File(std::string filename, std::size_t size) : filename(filename), size(size) {}
   File(const File &other) : filename(other.filename), size(other.size) {}

   inline std::size_t get_size() const { return this->size; }
};

class Directory
{
public:
   using Shared = std::shared_ptr<Directory>;

protected:
   std::vector<Directory::Shared> directories;
   std::vector<File> files;
   std::string label;
   std::size_t size;
   
public:
   using Shared = std::shared_ptr<Directory>;

   Directory(std::string label) : label(label), size(0) {}
   Directory(const Directory &other) : directories(other.directories), files(other.files), label(other.label), size(other.size) {}

   inline std::string get_label() { return this->label; }
   void add_directory(Directory::Shared dir) { this->directories.push_back(dir); this->size = 0; }
   void add_file(File &file) { this->files.push_back(file); this->size = 0; }
   std::size_t get_size()
   {
      if (this->size == 0) {
         for (auto dir : this->directories)
         {
            this->size += dir->get_size();
         }

         for (auto file : this->files)
         {
            this->size += file.get_size();
         }
      }

      return this->size;
   }
};

using Filesystem = std::map<std::string, Directory::Shared>;

Filesystem parse_filesystem()
{
   std::string input;
   std::vector<std::string> output;
   Filesystem result = { std::pair("/", std::make_shared<Directory>("/")) };
   std::vector<Directory::Shared> traversal;
   std::string current_directory;

   while (std::getline(std::cin, input))
   {
      output.push_back(input);
   }

   std::size_t index = 0;

   while (index < output.size())
   {
      std::string line = output[index];
      
      std::istringstream parse_shell;
      parse_shell.str(line);

      std::string dollar, command;
      parse_shell >> dollar >> command;

      if (command == "cd")
      {
         std::string arg;
         parse_shell >> arg;
         
         if (arg == "..") {
            traversal.pop_back();
            std::size_t folder_index = current_directory.rfind("/");
            current_directory = current_directory.substr(0, folder_index);
         }
         else {
            if (current_directory != "/" && arg != "/")
               current_directory += "/";

            current_directory += arg;

            traversal.push_back(result[current_directory]);
         }

         ++index;
      }
      else if (command == "ls")
      {
         for (++index; index < output.size() && output[index][0] != '$'; ++index)
         {
            std::istringstream parse_fs;
            parse_fs.str(output[index]);

            std::string size_or_dir, filename;
            parse_fs >> size_or_dir >> filename;

            if (size_or_dir == "dir")
            {
               std::string new_directory = current_directory;
               
               if (current_directory == "/")
                  new_directory += filename;
               else
                  new_directory += "/" + filename;
               
               auto new_node = std::make_shared<Directory>(filename);
               result.insert(std::pair(new_directory, new_node));
               result[current_directory]->add_directory(new_node);
            }
            else
            {
               std::size_t size = std::stoi(size_or_dir);
               result[current_directory]->add_file(File(filename, size));
            }
         }
      }
   }

   return result;
}

int main(int argc, char *argv[])
{
   auto filesystem = parse_filesystem();
   std::size_t size = 0;

   for (auto dir : filesystem)
   {
      auto dir_size = dir.second->get_size();
      
      if (dir_size < 100000)
         size += dir_size;
   }

   std::cout << "Part 1: " << size << std::endl;

   std::size_t space_taken = filesystem["/"]->get_size();
   std::size_t space_free = 70000000 - space_taken;
   std::size_t space_needed = 30000000 - space_free;
   std::size_t target_size = std::numeric_limits<std::size_t>::max();

   for (auto dir : filesystem)
   {
      auto dir_size = dir.second->get_size();

      if (dir_size > space_needed && dir_size < target_size)
         target_size = dir_size;
   }

   std::cout << "Part 2: " << target_size << std::endl;
}
