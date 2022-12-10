#include <iostream>
#include <string>
#include <stack>
#include <vector>
#include <fstream>
#include <algorithm>
#include <sstream>
#include <queue>

using namespace std;


using operation = vector<int>;  // an operation is a vector of int
using Stack  = stack<char>;      // stack of each value
 

void reverseStack(Stack & S)
{
  //reverse the stack
  queue<char> Q;

  while(S.size())
  {
    Q.push(S.top()); S.pop();
  }
  while(Q.size())
  {
    S.push(Q.front()); Q.pop();
  }
}

operation processLine(string & line)
{
  stringstream ss(line);
  string s;
  ss >> s;
  int a, b, c;
  ss >> a;
  ss >> s;
  ss >> b;
  ss >> s;
  ss >> c;

  return vector<int>{a, b, c};

}

void executeOperation(operation & operation, vector<Stack> & stacks)
{
  int numobjects = operation[0];
  auto &Left = stacks[operation[1] - 1];
  auto &Right = stacks[operation[2] - 1];
  stack<char> tmp;

  for(int i=0; i < numobjects; i++)
  {
    auto val = Left.top(); Left.pop();
    tmp.push(val);
  }

  while(tmp.size())
  {
    Right.push(tmp.top());
    tmp.pop();
  }
}
pair<vector<Stack>, vector<operation>> readInput(string &filename)
{
  //function to read the input file
  //readcing all the input in a stack to get the numbe of stack

  vector<operation> ops;
  vector<Stack> stacks;
  ifstream in(filename);
  int num_stacks = -1;
  string line;
  // reading the configuration 
  getline(in, line);
  while(line != "" && line[1]!='1')
  {
    if(num_stacks == -1){
      num_stacks = (line.size() + 1) / 4;
      stacks = vector<Stack>(num_stacks);
    }
    for(int i = 1; i < line.size(); i+= 4)
      if( line[i] != ' ')
        stacks[i/4].push(line[i]);

    //getting the next line
     getline(in, line);
  }

  //Now filling the operations
  getline(in, line);
  while(in)
  {
    if(line != "")
      ops.push_back(processLine(line));
    getline(in, line);
  }

  for(auto &S: stacks)
    reverseStack(S);
  in.close();
  return make_pair(stacks, ops);
}


int main (int argc, char *argv[])
{
  // string filename("./sample");
  string filename("input");
  auto [stacks, operations] = readInput(filename);
  for(auto operation: operations)
  {
    executeOperation(operation, stacks);
  }
  string result ="";
  for(auto  S : stacks)
    result += S.top();
  cout << result << endl;
  return 0;
}


