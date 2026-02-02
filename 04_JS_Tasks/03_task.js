let sum = 0;
function makeArrayAndPass() {
  const firstArr = Symbol("firstArr");

  const obj = {
    [firstArr]: [6, 4, 6, 4],
  };

  const firstElementOfFirstArray = obj[firstArr].shift();

  return mergeArray(firstElementOfFirstArray, obj[firstArr]);
}

function mergeArray(firstElementOfFirstArray, firstArr) {
  const secondArr = Symbol("secondArr");

  const obj = {
    [secondArr]: [5, 5, 5],
  };

  return [firstElementOfFirstArray, ...obj[secondArr], ...firstArr];
}

const secondArr = makeArrayAndPass();

const checkSumPromise = new Promise(function (resolve, rejest) {
  for (let i = 0; i < secondArr.length; i++) {
    sum += secondArr[i];
  }
  if (sum === 35) {
    resolve("Sum is 35");
  } else {
    rejest("Sum is not 35");
  }
});

checkSumPromise
  .then((data) => console.log(data))
  .catch((data) => console.log(data));

// const name = "PREM";

// const obj = {
//   name: "PREM",
// };

// const obj2 = {
//   [name]: "PREM",
// };

// console.log(obj);

// console.log(obj2);
