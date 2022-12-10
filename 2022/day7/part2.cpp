#include <iostream>
#include <string>
#include <sstream>
#include <vector>


using namespace std;
const int limit = 100000;
const int capacity = 70000000;
const int needed = 30000000;


//Creating the structure of node Tree
struct TreeNode
{
  string name;  // name of the node
  int size ;    // size of teh node

  vector<TreeNode*> children;   // set of children
  TreeNode * parent;            // parent of the node
  TreeNode(string name, TreeNode *parent=nullptr):name(name),parent(parent){};
};

void processCommand(string line, TreeNode * &current)
{
  // function to process a command or output 
  if( line[0] == '$')
  {
    //process command;
    if( line[2] == 'c' && line[3] == 'd'){
      //getting the name of the folder
      stringstream ss(line);
      char c ; 
      string name;
      ss >> c >> name >> name;
      if(name == "..")
        current = current->parent;
      else
        for(auto child : current->children)
        {
          if(child->name == name)
          {
            current = child;
            break;
          }
        }


    }

  } 
  else if (line.substr(0,3) == "dir")
  {
    // add a new children
    stringstream ss(line);
    string command;
    string name;
    ss >> command >>  name;
    current->children.push_back(new TreeNode(name, current));
  }
  else
  {
    // This is a file so increase the size of the current node
    stringstream ss(line);
    int size;
    ss >> size;
    current->size += size;

  }
}
void walk(TreeNode * root, vector<int> & alls)
{
  //function to recursive walk
  if(!root)
    return;
  for(auto child: root->children)
  {
    walk(child, alls);
    root->size += child->size;
  }
  alls.push_back(root->size);
}

int main (int argc, char *argv[])
{
  string line;  
  //reading the fire line
  getline(cin, line);
  getline(cin, line);
  auto root = new TreeNode("/");
  vector<int> alls;
  

  //Starting the node from root
  auto current = root;
  while(cin)
  {
    getline(cin, line);
    if(line != "")
      processCommand(line, current);
  }

  walk(root, alls);
  int free = capacity - root->size;
  int toDelete =  (needed - free);

  sort(alls.begin(), alls.end());
  for(auto v : alls)
    if( v >= toDelete)
    {
      cout << v << endl;
      break;
    }


  return 0;
}
