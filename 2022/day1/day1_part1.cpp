#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <algorithm>
#include <sstream>

using namespace std;

vector<int> readInput(string & filename)
{

  ifstream In(filename);
  vector<int> out;
  string number;
  int calory =0;
  while(In)
  {
    
    getline(In, number);
    if( number == "")
    {
      out.push_back(calory);
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
  make_heap(calories.begin(), calories.end());

  cout << calories.front() << endl;

  return 0;
}

