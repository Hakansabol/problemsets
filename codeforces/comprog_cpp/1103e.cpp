#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
int main() {
  int t;
  cin >> t;
  for (int _ = 0; _ < t; _++) {
    int n;
    cin >> n;
    std::vector<int> v(n);
    for (int _ = 0; _ < n; _++) {
      int c;
      cin >> c;
      v[_] = c;
    }
    // cout << v.size() << endl;

    vector<bool> validpairs(6003 * (n + 1), false);
    vector<bool> shishset(n + 1, false);
    int hsma{};
    int hsmi{};
    for (int i = 0; i < n; i++) {
      for (int idx = hsmi; idx <= hsma; idx++) {
        shishset[idx] = false;
      }
      hsma = v[i];
      hsmi = v[i];
      for (int j = i; j < n; j++) {
        int a = v[j];
        if (shishset[a]) {
          break;
        }
        hsma = max(hsma, a);
        hsmi = min(hsmi, a);
        if (hsma - hsmi == (j - i)) {
          validpairs[hsmi * 6001 + hsma] = true;
        }
      }
      // cout << validpairs.size() << endl;
    }
    int ans{};
    for (int a = 0; a < validpairs.size(); a++) {
      if (!validpairs[a])
        continue;
      int l = a / 6001;
      int r = a % 6001;
      int cap = r - l + 1;
      int val = (r + 1) * 6001 + cap + r;
      if (validpairs[val]) {
        ans = max(ans, cap);
      }
    }
    cout << ans << endl;
  }
}
