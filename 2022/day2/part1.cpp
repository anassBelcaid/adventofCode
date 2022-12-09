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
  auto move2 = getMove(m2);

  auto score = scoreMove(move2);
  int result = 0;

  // analysis for Rock
  if( move2 == ROCK)
  {
    if( move1 == ROCK)
      result = 3;
    else if ( move1 == PAPER)
      result = 0;
    else
      result = 6;
  }
  
  // analysis for Paper
  if( move2 == PAPER)
  {
    if( move1 == PAPER)
      result = 3;
    else if ( move1 == ROCK)
      result = 6;
    else
      result = 0;
  }

  // analysis for Scissor
  if( move2 == SCISSOR)
  {
    if( move1 == SCISSOR)
      result = 3;
    else if ( move1 == ROCK)
      result = 0;
    else
      result = 6;
  }

  return score + result;


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
    result += round;

  }
   cout << result << endl;

  
  return 0;
}
