let sum = 0;
function makeArrayAndPass() {
  let firstArr = [4, 6, 4];
  let firstElementOfFirstArray = firstArr.shift();
  return mergeArray(firstElementOfFirstArray, firstArr);
}

function mergeArray(firstElementOfFirstArray, firstArr) {
  let secondArr = [6, 5, 5, 5];
  return [firstElementOfFirstArray, ...secondArr, ...firstArr];
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
