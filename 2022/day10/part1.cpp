#include <iostream>
#include <string>
#include <sstream>
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
 }
 void updateCumul()
 {
   if(cycle % 40 == 20)
   {
     //update the cumul
     // cout << cycle << " * " << X << "= " << cycle * X << endl;
     signalCumul +=  cycle * X;
   }

 }
 void command(string &line)
 {

   if(line == "noop")
   {
     cycle++;
     updateCumul();
   }
   else if( line == "")
     return;
   else{

     stringstream ss(line);
     string word;
     int value;
     ss >> word >> value;

     cycle++ ; updateCumul();
     cycle++ ; X += value;  updateCumul();
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
  cout << C.signalCumul << endl;
  return 0;
}

