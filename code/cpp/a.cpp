#include <iostream>
#include <strstream>
using namespace std;
template <class T, int S>
class Node {
public:
   	T nodes[S];
	void print();
	Node() {
		int i;
		for (i=0;i<S;i++) {
			ostrstream a;
			a<<"apple "<<i;
			nodes[i]=a.str();
		}
	}
};

template <class T, int S>
void Node<T,S>::print()
{
	int i;
	for (i=0;i<S;i++) {
		cout<<i<<" = "<<nodes[i]<<endl;
	}
}


int main()
{
	Node<string,10> a;
	a.print();
   	return 0;
}
