func minCost(colors string, neededTime []int) int {
    ln:=len(colors)
    if ln<2 {
        return neededTime[0]
    }
    ans:=0
    ll:=0
    rr:=1
    for rr<ln{
        if colors[ll]==colors[rr] {
            if neededTime[ll]<=neededTime[rr]{
                ans+=neededTime[ll]
                ll=rr
                rr++
            } else {
                ans+=neededTime[rr]
                rr++
            }
        } else {
            ll=rr
            rr++
        }
    }
    return ans
}
