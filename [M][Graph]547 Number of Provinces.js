 /**
 * @param {number[][]} isConnected
 * @return {number}
 */
//DFS method
var findCircleNum = function(isConnected) {
    let n = isConnected.length;
    let arr = Array(n).fill(false);
    let cnt = 0;

    const dfs = (i) => {
        for (let j=0; j<n; j++){
            if (isConnected[i][j] == 1 && arr[j]==false){
                arr[j] = true;
                dfs(j);
            }
        }
    }

    for (let i=0; i<n; i++){
        if (arr[i]==false){
            dfs(i);
            cnt++;
        }

    }
    return cnt;
};
