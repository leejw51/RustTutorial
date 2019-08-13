#include <string>
#include <cstring>
#include <fstream>
#include <iostream>
#include <vector>
#include <cassert>
using namespace std;

typedef vector<string> Words;

Words mysort(Words src)
{
	if (src.size() <= 1)
		return src;

	int i = src.size() / 2;
	string pivot = src[i];

	Words left;
	Words right;
	for (auto &b : src)
	{
		auto g = b.compare(pivot);
		if (g < 0)
			left.push_back(b);
		else if (g > 0)
			right.push_back(b);
	}

	Words ret;
	auto sortedLeft = mysort(left);
	auto sorteRight = mysort(right);
	for (auto &i : sortedLeft)
	{
		ret.push_back(i);
	}
	ret.push_back(pivot);
	for (auto &i : sorteRight)
	{
		ret.push_back(i);
	}
	return ret;
}
int main()
{
	Words a;
	a.push_back("zoro");
	a.push_back("apple");
	a.push_back("question");
	a.push_back("counter");
	a.push_back("strike");
	a.push_back("delta");

	auto c = mysort(a);
	cout << "***************************" << endl;
	for (auto &b : c)
	{
		cout << b << endl;
	}

	return 0;
}