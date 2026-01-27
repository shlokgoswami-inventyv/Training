/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
function getPrice(pizza) {
  if(pizza==="Margherita") return 7;
  else if(pizza==="Caprese") return 9;
  else if(pizza==="Formaggio") return 10;
  else if(pizza==="ExtraSauce") return 1;
  else if(pizza==="ExtraToppings") return 2;
  
}
export function pizzaPrice(pizza, ...extras) {
  if(extras.length===0){
    return getPrice(pizza);
  }
  return getPrice(extras.shift())+pizzaPrice(pizza,...extras);
}

/**
 * Calculate the price of the total order, given individual orders
 *
 * (HINT: For this exercise, you can take a look at the supplied "global.d.ts" file
 * for a more info about the type definitions used)
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
  return pizzaOrders.reduce((acc,a)=>{
    return acc+pizzaPrice(a.pizza,...a.extras);
  },0)
}
