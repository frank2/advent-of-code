#include <iostream>
#include <limits>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

struct Node;
using SharedNode = std::shared_ptr<Node>;

struct Node
{
   struct Compare
   {
      bool operator() (SharedNode left, SharedNode right) const {
         std::uintptr_t l = reinterpret_cast<std::uintptr_t>(left.get());
         std::uintptr_t r = reinterpret_cast<std::uintptr_t>(right.get());

         return l < r;
      }
   };

   char label;
   std::map<SharedNode, int, Node::Compare> edges;

   Node() : label('a') {}
   Node(char label) : label(label) {}
   Node(const Node &other) : label(other.label), edges(other.edges) {}

   int height(SharedNode other) { return other->label - this->label; }
   void add_edge(SharedNode other) { this->edges.insert(std::make_pair(other, this->height(other))); }
};

struct Graph
{
   std::vector<SharedNode> nodes;
   SharedNode start, end;

   Graph() {}
   Graph(const Graph &other) : nodes(other.nodes), start(other.start), end(other.end) {}

   static Graph parse() {
      std::string line;
      std::vector<std::string> lines;

      while (std::getline(std::cin, line))
         lines.push_back(line);

      Graph result;

      for (std::size_t y=0; y<lines.size(); ++y)
      {
         auto row = lines[y];
         
         for (std::size_t x=0; x<row.size(); ++x)
         {
            auto node = std::make_shared<Node>(row[x]);

            if (node->label == 'S')
            {
               node->label = 'a';
               result.start = node;
            }
            else if (node->label == 'E')
            {
               node->label = 'z';
               result.end = node;
            }
               
            result.nodes.push_back(node);
            
            auto pos = y * row.size() + x;
            auto dx = pos - 1;
            auto dy = (y - 1) * row.size() + x;

            if (x != 0)
            {
               result.nodes[dx]->add_edge(result.nodes[pos]);
               result.nodes[pos]->add_edge(result.nodes[dx]);
            }
            if (y != 0)
            {
               result.nodes[dy]->add_edge(result.nodes[pos]);
               result.nodes[pos]->add_edge(result.nodes[dy]);
            }
         }
      }

      return result;
   }
   
   std::map<SharedNode, int, Node::Compare> traverse(bool downhill=false)
   {
      std::map<SharedNode, int, Node::Compare> distance;

      for (auto node : this->nodes)
         distance[node] = 0;
      
      auto queue = std::vector<SharedNode>();
      std::set<SharedNode, Node::Compare> visited;

      SharedNode start_node;

      if (downhill)
         start_node = this->end;
      else
         start_node = this->start;
      
      queue.push_back(start_node);
      visited.insert(start_node);
      distance[start_node] = 0;

      while (queue.size() > 0)
      {
         auto node = queue.front();
         queue.erase(queue.begin());
         
         if (downhill && node == this->start || !downhill && node == this->end)
            break;

         for (auto neighbor : node->edges)
         {
            if (visited.find(neighbor.first) != visited.end()) { continue; }
            if (downhill && neighbor.first->edges[node] > 1 || !downhill && neighbor.second > 1) { continue; }
            
            distance[neighbor.first] = distance[node]+1;
            visited.insert(neighbor.first);
            queue.push_back(neighbor.first);
         }
      }

      return distance;
   }
};

int main(int argc, char *argv[])
{
   auto graph = Graph::parse();
   auto uphill = graph.traverse();
   auto downhill = graph.traverse(true);

   std::cout << "Part 1: " << uphill[graph.end] << std::endl;

   int lowest = std::numeric_limits<int>::max();

   for (auto node : graph.nodes)
   {
      if (node->label != 'a') { continue; }
      
      auto result = downhill[node];

      if (result != 0 && result < lowest)
         lowest = result;
   }

   std::cout << "Part 2: " << lowest << std::endl;
   
   return 0;
}
