#include <iostream>
#include <list>
#include <string>
#include <vector>
#include <fstream>
#include <sstream>
#include <algorithm>

using namespace std;



struct Monkey
{
  //represent the rule of a monkey
  Monkey(){
    items = list<long long>();
  }
  list<long long> items;   // list of holded items
  char operation;    // my operation for update
  string operand;       // operand to manipulate the list
  int divideRule;    //integer to divide by
  int indexTrue;     // index monkey if true
  int indexFalse;    // index monkey if false;

  long long computeNewValue(long long old)
  {
    long long val;
    if( operand == "old")
      val = old;
    else
      val = stoll(operand);

    if( operation == '*')
      return old * val;
    if(operation == '+')
      return old + val;
    if(operation == '-')
      return old - val;
    if(operation == '/')
      return old / val;
    return 0;
  }


  void print()const
  {
    cout << "items ";
    for(auto v : items)
      cout << v << " ";
    cout << endl;
    cout << operation << ", " << operand << endl;
    cout << "Divisible " << divideRule << " true " << indexTrue << ",  false " << indexFalse << endl;
    cout << "--------------------------------" << endl;
  }
};

class Monkeys
{
  // Class to represent the monkeyes
  public:
  vector<Monkey> monkeys;
  vector<long long> inspected;
  long long ppcm; //common multiple
  Monkeys(){
    monkeys = vector<Monkey>();
    inspected = vector<long long>();
    ppcm = 1;
  }

  long long monkeyBusiness()
  {
    sort(inspected.begin(), inspected.end());

    int n = inspected.size();
    return inspected[n-1] * inspected[n-2];
  }
  void playMonkey(int index)
  {
    // function to execute the round a monkey [index]
    auto &M = monkeys[index];
    while(M.items.size())
    {
      inspected[index]++;
      auto old = M.items.front();
      M.items.pop_front();

      //compute the new val by the new rule
      auto newVal = M.computeNewValue(old);
      // cout << "val " << newVal << endl;

      //reduction
      // newVal /= 3;
      newVal %= ppcm;

      //now sendinging
      if(newVal % M.divideRule == 0)
      {
        // cout << "Sending " << newVal << " to " << M.indexTrue << endl;
        monkeys[M.indexTrue].items.push_back(newVal);
       }
      else
      {
        // cout << "Sending " << newVal << " to " << M.indexFalse << endl;
        monkeys[M.indexFalse].items.push_back(newVal);
      }
    }
  }
  void round()
  {
    for(int i=0; i < (int)(monkeys.size()); i++)
      playMonkey(i);

  }
  public:
  void readMonkeyData(ifstream & in)
  {
    Monkey M;
    // function to read the data of an
    string line;
    //line with the monkey index
    getline(in, line);

    //reading the starting items
    getline(in, line, ':');
    getline(in,line);
    stringstream ss(line);
    int value;
    char c;
    while(  ss >> value)
    {
      M.items.push_back(value);
      ss >> c;
    }
    // getting the update rule
    getline(in, line,'=');
    getline(in, line);
    string word;
    ss = stringstream(line);
    ss >> word >> M.operation >> M.operand;

    //getting the divisibility
    getline(in, line,':');
    getline(in, line);
    ss = stringstream(line);
    ss >> word >> word >> M.divideRule;

    //getting the two monkeys index
    getline(in, line,':');
    getline(in, line);
    ss = stringstream(line);
    ss >> word >> word >> word >> M.indexTrue;

    getline(in, line,':');
    getline(in, line);
    ss = stringstream(line);
    ss >> word >> word >> word >> M.indexFalse;

    getline(in, line);


    monkeys.push_back(M);
    inspected.push_back(0);
    ppcm *= M.divideRule;

  }

};
int main (int argc, char *argv[])
{
  
  // string filename = "sample";
  string filename = "input";
  ifstream in(filename);
  Monkeys game;
  while(!in.fail())
  {
     game.readMonkeyData(in);
  }
  for(int i=0; i < 10000; i++)
    game.round();

  for(int i=0; i < game.monkeys.size(); i++)
  {
    cout << "Monkey " << i << " : ";
    // for(auto v : game.monkeys[i].items)
    //   cout << v << " ";
    // cout << endl;
    cout << game.inspected[i] << endl;
  }
  cout << game.monkeyBusiness() << endl;
  return 0;
}
