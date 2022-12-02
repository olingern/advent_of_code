import { readFile } from "./src/common";

import { getPartOneAnswer } from "./src/day_2";

function main() {
  console.log("========== Advent of Code 2022 ==========");

  const dayTwoInput = readFile("./inputs/day_2.txt");
  console.log(dayTwoInput);

  console.log(getPartOneAnswer(dayTwoInput));
}

main();
