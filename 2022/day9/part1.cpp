#include <iostream>
#include <string>
#include <set>


using namespace std;


/*
 * Class to model the position of the Rob
 */
using position = pair<int,int>;
class Rob{

  public:

    Rob()
    {
      //starting position
      head = make_pair(0,0);
      tail = make_pair(0,0);
      visited = set<position>();
      visited.insert(make_pair(0,0));
    }

  void printState()const
  {
    cout << "H(" << head.first << ", " << head.second << ")\t";
    cout << "T("  << tail.first << ", " << tail.second << ")\n";
    
  }
  void right(int steps)
  {
    for(int i=0; i < steps; i++)
    {
      head.first++;

      if(head.first - tail.first == 2)
      {
        //move the tail
        tail.first ++;

        //move the tail second
        if(tail.second != head.second )
          tail.second += (tail.second < head.second) ? 1 : -1;

      }
      visited.insert(tail);
    }
  }
  void left(int steps)
  {
    for(int i=0; i < steps; i++)
    {
      head.first--;
      if(tail.first - head.first == 2)
      {
        tail.first --;
        if( tail.second != head.second)
        {
          tail.second += (tail.second < head.second) ? 1 : -1;
        }
      }
      visited.insert(tail);
    }
  }
  void up(int steps)
  {
    for(int i=0; i < steps; i++)
    {
      head.second ++;
      if( head.second - tail.second == 2)
      {
        tail.second ++;
        if( head.first != tail.first)
        {
          tail.first += (tail.first < head.first) ?  1 : -1;
        }
      }
      visited.insert(tail);
    }
  }
  void down(int steps)
  {
    for(int i = 0; i < steps; i++)
    {
      head.second--;
      if( tail.second - head.second == 2)
      {
        tail.second--;
        if( tail.first != head.first)
          tail.first += (head.first < tail.first) ? -1 : 1;

      }
      visited.insert(tail);
    }
  }


  int numVisited()const
  {
    return visited.size();
  }
  public:
    position head;   // positin of the head
    position tail;   // position of the tail
    set<position> visited;

};

int main (int argc, char *argv[])
{

  Rob R;  
  char direction;
  int steps;
  while(cin >> direction >> steps)
  {
    if(direction == 'R')
      R.right(steps);
    if (direction == 'L')
      R.left(steps);
    if( direction == 'U')
      R.up(steps);
    if( direction == 'D')
      R.down(steps);
  }
  // R.printState();

  cout << R.numVisited() << endl;


  return 0;

}
