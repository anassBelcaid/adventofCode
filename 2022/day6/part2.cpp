#include <iostream>
#include <string>
#include <sstream>
#include <unordered_map>


using cache = std::unordered_map<char, int>;

void removeKey(cache &C, char c)
{
  // function to remove a key from the map
  if( C[c] > 1)
    C[c]--;
  else
    C.erase(c);
}

using namespace std;


int solve(string &line)
{
  int i = 14;
  cache window;
  //pushing the first four character
  for(int j =0; j < 14; j++)
    window[line[j]]++;
  while( i < line.size())
  {
    if(window.size() == 14)
      return i;
    
    removeKey(window, line[i-14]);
    window[line[i++]]++;
  }
  return i + 1;

}

int main (int argc, char *argv[])
{
  string line;
  getline(cin, line);

  cout << solve(line) << endl;

  return 0;
}

