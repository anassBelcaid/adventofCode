#include<iostream>
#include<sstream>
#include<string>
#include <fstream>

using namespace std;

class List
{
  public:
  List(string line){};

  bool operator<(const List &other)const
  {
    return true;
  }
};

int main (int argc, char *argv[])
{
  string filename{"sample"};

  ifstream in(filename);
  string line;

  int index = 1;
  int sumIndex = 0;
  while(in)
  {
    //reading the first line
    getline(in, line);
    if(line == "")
      break;

    List L1(line);
    cout << line << endl;


    //reading the second list
    getline(in, line);
    cout << line << endl;
    List L2(line);
    
    if( L1 < L2)
       sumIndex += index;
    index++;
    getline(in, line);
  }


  in.close();
  return 0;
}
