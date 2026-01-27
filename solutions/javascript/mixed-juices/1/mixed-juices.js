// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Determines how long it takes to prepare a certain juice.
 *
 * @param {string} name
 * @returns {number} time in minutes
 */
export function timeToMixJuice(name) {
  switch(name){
    case "Pure Strawberry Joy":
      return 0.5;
      break;
    case "Energizer":
      return 1.5;
      break;
    case "Green Garden":
      return 1.5;
      break;
    case "Tropical Island":
      return 3;
      break;
    case "All or Nothing":
      return 5;
      break;
    default:
      return 2.5;
  }
}

/**
 * Calculates the number of limes that need to be cut
 * to reach a certain supply.
 *
 * @param {number} wedgesNeeded
 * @param {string[]} limes
 * @returns {number} number of limes cut
 */
export function limesToCut(wedgesNeeded, limes) {
  if(wedgesNeeded===0 || limes.length===0) return 0;
  let ans=0;
  let pos=0;
  for(let i=0;i<limes.length;i++){
    if(limes[i]==='small'){ans+=6;}
    else if(limes[i]==='medium'){ans+=8;}
    else{ans+=10;}
    pos=i;
    if(ans>=wedgesNeeded){break;}
  }
  return pos+1;
}

/**
 * Determines which juices still need to be prepared after the end of the shift.
 *
 * @param {number} timeLeft
 * @param {string[]} orders
 * @returns {string[]} remaining orders after the time is up
 */
export function remainingOrders(timeLeft, orders) {
  let ans=[];
  // console.log(timeLeft+"   hahaha  ");
    for(let i=0;i<orders.length;i++){
      
         timeLeft-=timeToMixJuice(orders[i]);
      
      
        if(timeLeft<0){
          ans.push(orders[i]);
        }
        if(timeLeft + timeToMixJuice(orders[i])>0){
          ans.pop();
        }
        
          console.log("inside----",timeLeft,ans);
    }
  // console.log("ans---",ans)

  return ans;
}
