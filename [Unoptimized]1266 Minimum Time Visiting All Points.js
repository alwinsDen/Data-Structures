/**
 * @param {number[][]} points
 * @return {number}
 */
var minTimeToVisitAllPoints = function(points) {
    let tmt = 0;
    if (points.length==0){
        return 0;
    }
    for(let [index, a] of points.entries()){
        if (index!=points.length-1){
            let iEs = [...a];
            let piv = false;
            if (a[0] < points[index+1][0] || a[1] < points[index+1][1]){
                piv = true;
            }
            while(iEs[0]!=points[index+1][0] || iEs[1]!=points[index+1][1]){
                console.log(iEs);
                if (piv){
                    if(iEs[0]==points[index+1][0]){
                        iEs[1]+=1;
                    } else if(iEs[1]==points[index+1][1]){
                        iEs[0]+=1;
                    } else {
                        iEs[0]+=1;
                        iEs[1]+=1;
                    }
                    tmt+=1;
                } else {
                    if(iEs[0]==points[index+1][0]){
                        iEs[1]-=1;
                    } else if(iEs[1]==points[index+1][1]){
                        iEs[0]-=1;
                    } else{
                        iEs[0]-=1;
                        iEs[1]-=1;
                    }
                    tmt+=1;
                }
            }
        }
    }
    return tmt;
};
