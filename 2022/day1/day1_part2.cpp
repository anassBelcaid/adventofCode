#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <algorithm>
#include <sstream>
#include <queue>

using namespace std;
using Queue= priority_queue<int,vector<int>, greater<int>>;

Queue readInput(string & filename)
{

  ifstream In(filename);
  Queue out;
  string number;
  int calory =0;
  while(In)
  {
    
    getline(In, number);
    if( number == "")
    {
      if( out.size() < 3)
        out.push(calory);
      else if( calory > out.top())
      {
        out.pop();
        out.push(calory);
      }
      calory = 0;
    }
    else
    {
      calory += stoi(number);
    }
  }
  return out;

}

int main (int argc, char *argv[])
{
  
  string filename{"input.txt"};
  auto calories = readInput(filename);

  auto sum = 0;
  while(calories.size())
  {
    sum += calories.top(); calories.pop();
  }
  cout << sum << endl;

  return 0;
}

