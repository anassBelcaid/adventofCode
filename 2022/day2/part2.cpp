#include <iostream>
#include <string>
#include <fstream>

using namespace std;

enum move{ROCK = 0, PAPER = 1, SCISSOR = 2};

enum move getMove(char c)
{
  //function to get the encoded hand
  if( c == 'A' || c == 'X')
    return ROCK;
  if ( c== 'B' || c == 'Y')
    return PAPER;

  return SCISSOR;
}

int scoreMove(enum move &m)
{
  //function to score the move
  if( m == ROCK)
    return 1;
  if( m == PAPER)
    return 2;

  return 3;
}

int scoreRound( char m1, char m2)
{
  auto move1 = getMove(m1);
  enum move move2;

  //we need to lose
  if( m2 == 'X')
  {
    if(move1 == ROCK)
      move2 = SCISSOR;
    else if (move1 == PAPER)
      move2 = ROCK;
    else
      move2 = PAPER;

    return scoreMove(move2);

  }

  // we need to draw
  else if ( m2  == 'Y')
  {
      return  3 + scoreMove(move1);
  }
  //we need to win
  else
  {

    if(move1 == ROCK)
      move2 = PAPER;
    else if (move1 == PAPER)
      move2 = SCISSOR;
    else
      move2 = ROCK;

    return 6 + scoreMove(move2);

  }



}

int main (int argc, char *argv[])
{
  string filename = "input";
  ifstream in(filename);
  char c1, c2;
  int result = 0;

  while( in >> c1 >> c2 )
  {

    auto round = scoreRound(c1, c2);
    // cout << round << endl;
    result += round;

  }
   cout << result << endl;

  
  return 0;
}
