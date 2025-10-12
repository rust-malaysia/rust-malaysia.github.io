#include <iostream>
#include <vector>
using namespace std;

int main() {
    vector<int> v = {1, 2, 3};
    int *num = &v[2];
    v.push_back(4); // <-- may reallocate and invalidate 'num'
    cout << *num << endl;
}

