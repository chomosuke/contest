#include <iostream>
#include <climits>

using namespace std;

int main()
{
    int t;
    cin >> t;
    for (int j = 0; j < t; j++) {
        int n, m, p;
        cin >> n >> m >> p;
        int* s = new int[n];
        int* e = new int[n];
        int* t = new int[n];
        for (int i = 0; i < n; i++) {
            cin >> s[i] >> e[i] >> t[i];
        }

        int* distance = new int[p];
        distance[0] = 0;
        for (int i = 1; i < p; i++) {
            distance[i] = INT_MAX;
        }

        for (int k = 0; k < p - 1; k++) {
            for (int i = 0; i < n; i++) {
                if (distance[e[i]] != INT_MAX && distance[e[i]] + t[i] < distance[s[i]]) {
                    distance[s[i]] = distance[e[i]] + t[i];
                }
            }
        }

        int count = 0;
        for (int i = 1; i < p; i++) {
            if (distance[i] <= -m) {
                count++;
            }
        }
        cout << count << endl;
    }
}