#include <iostream>
#include <string>
#include <algorithm>
#include <set>
#include <fstream>
#include <ctype.h>


using namespace std;

int solve(set<char> &part1, set<char> & part2, set<char> & part3)
{
  
  for(auto v :part1)
  {
    auto test1 = (part2.find(v) != part2.end());
    auto test2 = (part3.find(v) != part3.end());
    if(test1 && test2)
      return (islower(v)) ? 1 + (v - 'a') : 27 + ( v - 'A');
  }
  return 0;
}

int main (int argc, char *argv[])
{
  

  string rucksack;
  set<char> part1, part2, part3;
  int result = 0;

  while ( cin >> rucksack)
  {
    if( rucksack != "")
    {
      //getting the next 
      part1 = set<char>(rucksack.begin(), rucksack.end());
      cin >> rucksack;
      part2 = set<char>(rucksack.begin(), rucksack.end());
      cin >> rucksack;
      part3 = set<char>(rucksack.begin(), rucksack.end());
      result += solve(part1, part2, part3);
    }
  }
  cout << result << endl;
  return 0;
}

