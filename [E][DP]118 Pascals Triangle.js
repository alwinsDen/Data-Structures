/**
 * @param {number} numRows
 * @return {number[][]}
 */
var generate = function(numRows) {
    
    let ans = [[1]];
    for(let i=1; i < numRows; i++){
        let pr = [];
        for ( j = 0; j < i + 1; j++ ){
            if ( j == 0 ) {
                pr.push(ans[i-1][0])
            } else if (j==i){
                pr.push(ans[i-1][(ans[i-1]).length - 1])
            } else {
                pr.push( ans[i-1][j - 1] + ans[i-1][j] )
            }
        }
        ans.push(pr);
    }
    return ans;
};
