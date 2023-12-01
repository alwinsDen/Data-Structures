/**
 * @param {number} n
 * @return {number[]}
 */
var countBits = function(n) {
    let number = n + 1;
    let ans = [];
    while(number>=0 && number--){
        let nmb = number;
        let wns = 0;
        while(nmb){
            if (nmb & 1) {
                wns++;
            } 
            nmb >>= 1;
        }
        ans.push(wns);
    }
    ans.reverse();
    return ans;
};
