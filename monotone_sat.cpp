#include<bits/stdc++.h>
using namespace std;

constexpr int n = 3;
constexpr int m = 7;
constexpr int K = 3;
constexpr int T = 2;

int cnt[T + 1][(1 << n)];
bool vio[T + 1][n];
bool var[m + 1];

int sat[n][K] = {
    {1, 2, -3},
    {3, 4, 5},
    {-5 ,6, 7}
};

int random_space = 0;
int pt = 0;

void init_random_space(int x) {
    random_space = x; pt = 0;
}

bool fetch_random_bit() {
    return ( random_space >> pt ++ ) & 1;
}

void init_sat() {
    for( int i = 1; i <= m; i ++ ) 
        var[i] = fetch_random_bit();
}

bool sat_checker(int x) {
    bool ans = 1;
    for( int k = 0; k < K; k ++ ) 
        ans &= (sat[x][k] < 0) ^ var[abs(sat[x][k])];
    return ans;
}

void sat_resampling(set<int> C) {
    set<int> V;
    for( auto c : C ) for( int k = 0 ; k < K; k ++ )
        V.insert(abs(sat[c][k]));
    for( auto v : V ) 
        var[v] = fetch_random_bit();
}

int main() {
    int bits = (T + 1) * m;
    int all = 0;
    for( int r = 0; r < (1 << bits); r ++) {
        if( (r + 1) % (1 << (bits - 7)) == 0 ) cout << (r + 1) / (1 << (bits - 7)) << " / 128\n";

        init_random_space(r);
        init_sat();
        
        for( int j = 0; j < n; j ++ ) 
            vio[0][j] = sat_checker(j);
        
        for( int t = 1; t <= T; t ++ ) {
            bool terminal = 1;
            for( int j = 0; j < n; j ++ )
                terminal &= !vio[t - 1][j];
            if( terminal ) break;
            
            set<int> C;
            for( int j = 0; j < n; j ++ ) if( vio[t - 1][j] )
                C.insert(j);
            sat_resampling(C);
            
            for( int j = 0; j < n; j ++ )  
                vio[t][j] = sat_checker(j); 
                
            for( int s = 1; s < (1 << n); s ++ ) {
                int ans = 1;
                for( int j = 0; j < n; j ++ ) if( (s >> j) & 1 ) ans &= vio[t][j];
                cnt[t][s] += ans;
            }

            if( t == 2 and vio[0][1] and vio[1][0] and vio[1][2] ) all ++; 
        }
    }
    for(int t = 1; t <= T; t ++ ) for( int s = 1; s < (1 << n); s ++ ) 
        cout << bits - log2(cnt[t][s]) << " \n"[s == (1 << n) - 1];

    cout << bits - log2(all) << "\n";
    return 0;
}
