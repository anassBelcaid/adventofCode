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


  }
public:
  int maxScore()const
  {
    //function to compute the maximum score of each trees
    int score = 0;
    for(int i=0; i < n ; i++)
      for(int j = 0; j < m ; j++)
      {
        auto S = scenicScore(i,  j);
        score = (score < S ) ? S : score;
        
      }
    return score;
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

  int scenicScore(int i, int j)const
  {
    // compute the scenic score of the point (i, j)
    auto ref = trees[i][j];

    //right;
    int right = 0;
    int i1=i;
    int j1 = j+1;
    while( j1 < m && trees[i1][j1] < ref)
    {
      j1++;
      right++;
    }
    if( j1 < m && trees[i1][j1] >= ref)
      right++;
    //left
    int left = 0;
    j1 = j-1;
    while( j1 >= 0 && trees[i1][j1] < ref)
    {
      j1--;
      left++;
    }
    if( j1 >=0 &&  trees[i1][j1] >= ref)
      left++;


    // up 
    int  up = 0;
    i1 = i-1;
    j1 = j;
    while( i1 >= 0 && trees[i1][j1] < ref )
    {
      i1--;
      up++;

    }
    if(i1 >= 0 && trees[i1][j1] >= ref)
      up++;

    //down 
    int   down = 0;
    i1 = i+1;
    j1 = j;
    while( i1 < n && trees[i1][j1] < ref )
    {
      i1++;
      down++;

    }
    if( i1 < n && trees[i1][j1] >= ref)
       down++;


    // cout << "UP " << up << "  down :" << down << " left "<< left << " Rigth " << right << endl;
    return up * left * right * down;
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



  //printing the scenic score
  cout << S.maxScore() << endl;

  return 0;
}







