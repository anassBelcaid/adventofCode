#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>
#include <queue>
#include <math.h>


using namespace std;
using grid = vector<vector<int>>;
using position = pair<int, int>;
using state = pair<position, int>;

class Path
{
  /*
   * class to represent the path to existing goal
   */
  public:
  Path(ifstream & in)
  {
    
    G = grid();
    string line;
    getline(in, line);
    //reading the line
    
    int i=0;
    while(line != "")
    {
      processLine(line, i++);
      getline(in, line);
    }
    n = G.size();
    m = G[0].size();
    visited = vector(n, vector(m, false));
    frontier = queue<state>();
    
  }

  vector<position> neighbors(position & pos)
  {
    auto [x, y] = pos;
    //getting the height
    auto h = G[x][y];

    vector<position> N;


    //left
    if( x > 0 && !visited[x-1][y])
    {
      auto h1 = G[x-1][y];
      if( h <= h1 || h -h1 == 1)
        N.push_back(make_pair(x - 1, y));
    } 
    //right point
    if( x < n - 1 && !visited[x+1][y])
    {
      auto h1 = G[x+1][y];
      if(h <= h1 || h - h1 == 1)
        N.push_back(make_pair(x+1, y));
    }

    //up 
    if( y > 0 && !visited[x][y-1])
    {
      auto h1 = G[x][y-1];
      if(h  <= h1 || h - h1 == 1)
        N.push_back(make_pair(x, y-1));
    }

    //down
    if( y < m - 1 && !visited[x][y+1])
    {
      auto h1 = G[x][y+1];
      if(h <= h1 || h - h1 == 1)
        N.push_back(make_pair(x, y + 1));
    }


    return N;

  }
  int trailLenght()
  {
    //function to get the lenght of trail from the goal to any position with heigth = 0

    //reseting the visited node
    visited = vector(n, vector(m, false));

    frontier.push(make_pair(goal, 0));

    while(frontier.size())
    {
      //getting the current position
      auto [P, steps] = frontier.front();
      frontier.pop();

      if(visited[P.first][P.second])
        continue;
      

      if( G[P.first][P.second] == 0)
        return steps;

      auto N = neighbors(P);
      for(auto n : N)
        frontier.push(make_pair(n, steps+1));

      visited[P.first][P.second] = true;

    }
    return -1;
  }
  void processLine(string & line, int i)
  {
    //introduce the elevation of the line in the grid
    int n = line.size();
    vector<int> vals;
    for(int j=0; j < n; j++)
    {
      auto c = line[j];
      if(c == 'S')
      {
        start = make_pair(i, j);
        vals.push_back(0);
      }
      else if (c == 'E')
      {
        goal = make_pair(i,j);
        vals.push_back(25);
      }
      else
        vals.push_back(c - 'a');
    }
    G.push_back(vals);
  }
  public:
    position start;  // starting position
    position goal; // end position
    vector<vector<bool>> visited;
    queue<state> frontier;
    grid G;   // vector of the grid heigh
    int n;   // size of the grid
    int m; 

};

int main (int argc, char *argv[])
{
  // string filename{"sample"};
  string filename{"input"};
  auto in =  ifstream(filename) ;
  Path P(in);

  cout << P.trailLenght() << endl;
  
  in.close();
  return 0;
}
