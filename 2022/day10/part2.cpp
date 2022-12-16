#include <iostream>
#include <string>
#include <sstream>
#include <math.h>
#include <queue>

using namespace std;

// convention of an event which is a pair or <time, value>

class CPU
{
  public:
 CPU()
 {
   X = 1;
   cycle = 1;
   signalCumul = 0;
   update();
 }
 void update()
 {
   cout << getSymbol(); 
   if(cycle % 40 == 0)
   {
     cout << endl;
   }

 }
 
 char getSymbol()
 {
   // function to detect the positin of the Sprite
   
   //getting the pixel
   auto pixel = (cycle -1) % 40;
   if( fabs(X - pixel) <= 1)
     return '#';
   else
     return '.';

 }
 void command(string &line)
 {

   if(line == "noop")
   {
     cycle++;
     update();
   }
   else if( line == "")
     return;
   else{

     stringstream ss(line);
     string word;
     int value;
     ss >> word >> value;

     cycle++ ; update();
     cycle++ ; X += value;  update();
   }

   
 }
 public:
   int X;          // value of X
   int cycle; // time of the simulation
   int signalCumul;

};

int main (int argc, char *argv[])
{

  CPU C;
  string line;


  while(getline(cin, line))
  {
    if(line != "")
    C.command(line);
  }
  return 0;
}

