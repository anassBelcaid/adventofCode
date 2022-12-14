#include <iostream>
#include <string>
#include <vector>
#include <math.h>
#include<set>
#define sign(x)( (x>=0) ? 1 : -1)

using namespace std;
using position = pair<int,int>;

struct Node
{
  position P;
  Node * next;  //pointer to the next node

  Node():P(0,0),next(nullptr){}; //constructor empty
  Node(Node * next):P(0,0),next(next){};  // constructor with next
                                          //
  void print()const
  {
    cout << "("  << P.first << ", " << P.second << ")" << endl;
  }
  void up()
  {
    P.second ++;
    positionChanged();
  }
  void down()
  {
    P.second--;
    positionChanged();
  }
  void left()
  {
    P.first--;
    positionChanged();
  }
  void right()
  {
    P.first++;
    positionChanged();
  }

  //how to model the change in position for the next
  void positionChanged()  //complicated function to send the adapt the next
  {

    if(next)
    {
      //nothing to do 
      if(P.first == next->P.first && P.second == next->P.second)
        return;
      //check if same x 
      if( P.first == next->P.first)
      {
        auto diff = P.second - next->P.second;
        if( fabs(diff) >= 2)
          next->P.second += sign(diff);
      }

      //check is same y
      else if( P.second == next->P.second)
      {
        auto diff = P.first - next->P.first;
        if(fabs(diff)>=2)
          next->P.first += sign(diff);
      }
      else //both coordinates are differnt
      {

        auto diffX = P.first - next->P.first;
        auto diffY = P.second - next->P.second;
        if( fabs(diffX)>=2 || fabs(diffY)>=2)
        {
          next->P.first += sign(diffX);
          next->P.second += sign(diffY);

        }

      }

      //sending the next signal
      next->positionChanged();

    }

  }
};

//structure of teh rob with many nodes
struct Rob
{
  set<position> visited;
  Node* head;
  Node* tail;

  Rob(int n)
  {
    tail = new Node;
    head = tail;

    for(int i=0; i < n -1; i++)
    {
      auto tmp = new Node(head);
      head = tmp;
    }
  }
  void up()
  {
    head->up();
    visited.insert(tail->P);
  }
  void down()
  {
    head->down();
    visited.insert(tail->P);
  }
  void left()
  {
    head->left();
    visited.insert(tail->P);
  }
  void right()
  {
    head->right();
    visited.insert(tail->P);
  }

  int numVisited()const
  {
    return visited.size();
  }

};

int main (int argc, char *argv[])
{
  
  Rob R(10);


  char direction;
  int steps;

  while(cin >> direction >> steps)
  {
    if(direction == 'R')
      for(int i=0; i < steps; i++)
        R.right();
    if (direction == 'L')
      for(int i=0; i < steps; i++)
        R.left();
    if( direction == 'U')
      for(int i=0; i < steps; i++)
        R.up();
    if( direction == 'D')
      for(int i=0; i < steps; i++)
        R.down();
  }
  cout << R.numVisited() << endl;


  return 0;
}
