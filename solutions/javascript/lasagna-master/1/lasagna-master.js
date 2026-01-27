/// <reference path="./global.d.ts" />
// @ts-check


 
 export function cookingStatus(val) {
    if(!val && val!==0) return 'You forgot to set the timer.'
   else if(val===0) return 'Lasagna is done.'
   return 'Not done, please wait.';
  }
 export function preparationTime(layers,num=2) {
   return num*layers.length;
 }
export function quantities(arr) {
  let ans={noodles:0,sauce:0};
  arr.map((a,i)=>{
    if(a==="noodles"){
      ans[a]+=50
    }
    if(a==="sauce") ans[a]+=0.2
  })
   return ans;
 }
export function addSecretIngredient(friendsList,myList) {
   myList.push(friendsList[friendsList.length-1]);
 }
export function scaleRecipe(recipe,num) {
   let ans={};
    for(let key in recipe){
      ans[key]=recipe[key]*(num/2)
    }
  return ans;
 }
