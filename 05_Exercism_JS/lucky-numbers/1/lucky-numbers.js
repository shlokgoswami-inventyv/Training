// @ts-check

/**
 * Calculates the sum of the two input arrays.
 *
 * @param {number[]} array1
 * @param {number[]} array2
 * @returns {number} sum of the two arrays
 */
export function twoSum(array1, array2) {
  return Number(array1.reduce((acc,a)=>{return acc+a},"")) +
  Number(array2.reduce((acc,a)=>{return acc+a},""));
}

/**
 * Checks whether a number is a palindrome.
 *
 * @param {number} value
 * @returns {boolean} whether the number is a palindrome or not
 */
export function luckyNumber(value) {
  value=String(value);
  let flag=true;
  for(let i=0;i<value.length/2;i++){
    if(value[i]!==value[value.length-1-i]){flag=false;break;}
  }
  return flag;
}

/**
 * Determines the error message that should be shown to the user
 * for the given input value.
 *
 * @param {string|null|undefined} input
 * @returns {string} error message
 */
export function errorMessage(input) {
  if(!input || input=='') return 'Required field';
  else if(Number.isNaN(Number(input))) return 'Must be a number besides 0';
  return Number(input)===0 ? 'Must be a number besides 0'  : "";
}
