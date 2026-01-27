// @ts-check

/**
 * Determine how many cards of a certain type there are in the deck
 *
 * @param {number[]} stack
 * @param {number} card
 *
 * @returns {number} number of cards of a single type there are in the deck
 */
export function cardTypeCheck(stack, card) {
  let ans=0;
  stack.forEach((a)=>{a===card?ans++:true});
  return ans;
}

/**
 * Determine how many cards are odd or even
 *
 * @param {number[]} stack
 * @param {boolean} type the type of value to check for - odd or even
 * @returns {number} number of cards that are either odd or even (depending on `type`)
 */
export function determineOddEvenCards(stack, type) {
  let ans=0;
  for(let a of stack){
    (type && a%2==0)? ans++:true;
    (!type && a%2==1)? ans++:true;
  }
  return ans;
}
