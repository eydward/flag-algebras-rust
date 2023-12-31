#include <bits/stdc++.h>
using namespace std;
using ll = long long;
using pii = pair<int, int>;
#define mp make_pair
#define F first
#define S second

int c2(int n) { return n * (n - 1) / 2; }

int main() {
    ofstream fout("forbidden_digraphs");

    int a, b;
    cin >> a >> b;

    for (ll bm = 0; bm < (1 << c2(a)); bm++) {
        stringstream str;
        str << a + b << " : ";

        for (int i = 0; i < a; i++) {
            for (int j = i + 1; j < a; j++) {
                str << "(" << i << "," << j << "); ";
            }
        }

        for (int i = 0; i < a; i++) {
            for (int j = a; j < a + b; j++) {
                str << "(" << i << "," << j << "); ";
                str << "(" << j << "," << i << "); ";
            }
        }

        for (int i = a; i < a + b; i++) {
            for (int j = i + 1; j < a + b; j++) {
                str << "(" << i << "," << j << "); ";
                str << "(" << j << "," << i << "); ";
            }
        }

        int ct = 0;
        for (int i = 0; i < a; i++) {
            for (int j = 0; j < i; j++) {
                if (bm & (1 << ct)) {
                    str << "(" << i << "," << j << "); ";
                }
                ct++;
            }
        }

        fout << str.str().substr(0, str.str().size() - 2) << endl;
    }

    fout.close();
    return 0;
}
