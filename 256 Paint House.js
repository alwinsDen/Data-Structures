/**
 * @param {number[][]} costs
 * @return {number}
 */
var minCost = function(costs) {
    let n = costs.length;
    let arr_mat = Array.from({length: n},()=>Array(3).fill(0));
    for(let i=0;i<3;i++){
        arr_mat[0][i] = costs[0][i];
    }
    for(let i=1;i<n;i++){
        arr_mat[i][0] = costs[i][0] + Math.min(arr_mat[i-1][1], arr_mat[i-1][2]);
        arr_mat[i][1] = costs[i][1] + Math.min(arr_mat[i-1][2], arr_mat[i-1][0]);
        arr_mat[i][2] = costs[i][2] + Math.min(arr_mat[i-1][0], arr_mat[i-1][1]);
    }
    return Math.min(arr_mat[n-1][0], arr_mat[n-1][1], arr_mat[n-1][2]);
};
