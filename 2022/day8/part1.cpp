#include <iostream>
#include <string>
#include <vector>

using namespace std;
using matrix = vector<vector<int>>;
using bmatrix = vector<vector<bool>>;

class Solution
{
public:
  Solution()
  {
    trees = matrix();
    string line;
    while( cin >> line)
    {
      trees.push_back(getheights(line));
    }
    this->n = trees.size();
    this->m =  trees[0].size();
    visited = vector<vector<bool>>(n, vector<bool>(m, false));


    //visible from the left
    fromLeft();
    fromRight();
    fromUp();
    fromBelow();

  }
  int getVisible()const
  {
    int count = 0;
    for(int i=0; i < n; i++)
      for(int j= 0; j < m; j++)
        count += visited[i][j];
    return count;
  }
private:
  void fromLeft()
  {
    //markign true the trees visited from left
    for(int i=0; i < n ; i++)
    {
      auto maxval = trees[i][0];
      visited[i][0] = true;

      for(int j = 1; j < m - 1; j++)
      {
        if(trees[i][j] > maxval)
          visited[i][j] = true;

        maxval = (maxval > trees[i][j]) ? maxval : trees[i][j];

      }

    }
  }
  void fromUp(){

    //markign true the trees visited from left
    for(int j=0; j < m ; j++)
    {
      auto maxval = trees[0][j];
      visited[0][j] = true;

      for(int i = 1; i < n - 1; i++)
      {
        if(trees[i][j] > maxval)
          visited[i][j] = true;

        maxval = (maxval > trees[i][j]) ? maxval : trees[i][j];

      }

    }
  }

  void fromBelow(){

    //markign true the trees visited from left
    for(int j=0; j < m ; j++)
    {
      auto maxval = trees[n-1][j];
      visited[n-1][j] = true;

      for(int i = m-2; i > 0; i--)
      {
        if(trees[i][j] > maxval)
          visited[i][j] = true;

        maxval = (maxval > trees[i][j]) ? maxval : trees[i][j];

      }

    }

  }

  void fromRight()
  {

    //markign true the trees visited from left
    for(int i=0; i < n ; i++)
    {
      auto maxval = trees[i][m-1];
      visited[i][m-1] = true;

      for(int j = m-2; j > 0; j--)
      {
        if(trees[i][j] > maxval)
          visited[i][j] = true;

        maxval = (maxval > trees[i][j]) ? maxval : trees[i][j];

      }

    }

  }
  vector<int> getheights(string &line)
  {
    //function to conver the string into a vector of integers
    vector<int> row;
    for(int i=0; i < line.size(); i++)
      row.push_back(line[i] - '0');
    return row;

  }
  bool inBounds(int i , int j)const
  {
    if( i<0 || i >= n)
      return false;
    if( j<0 || j >= m)
      return false;
    return true;
  }
  
private:
  matrix trees;
  bmatrix visited;
  int n; 
  int m;
};



int main (int argc, char *argv[])
{

  //reading the input
  Solution S;

  cout << S.getVisible() << endl;

  return 0;
}







