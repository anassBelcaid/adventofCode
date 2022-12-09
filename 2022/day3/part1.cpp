#include <iostream>
#include <string>
#include <algorithm>
#include <set>
#include <fstream>
#include <ctype.h>


using namespace std;

int solve(set<char> &part1, set<char> & part2)
{
  for(auto c : part1)
    if(part2.find(c) != part2.end())
    {
      return (islower(c)) ? 1 + (c - 'a') : 27 + (c - 'A');
    }
  return 0;
}

int main (int argc, char *argv[])
{
  

  string rucksack;
  int result = 0;

  while ( cin >> rucksack)
  {
    if( rucksack != "")
    {
      int n = rucksack.size();
      auto it = rucksack.begin();
      auto part1 = set<char> (it , it + n/2);
      auto part2 = set<char> (it  + n/2, rucksack.end());
      auto rep = solve(part1, part2);
      // cout << rep << endl;
      result += rep;
    }
  }
  cout << result << endl;
  return 0;
}

