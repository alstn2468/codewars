// https://www.codewars.com/kata/556deca17c58da83c00002db/
export function tribonacci(
  [a, b, c]: [number, number, number],
  n: number,
): number[] {
  const result = [a, b, c];
  for (let index = result.length; index <= n; index++) {
    const newItem = result[index - 1] + result[index - 2] + result[index - 3];
    result.push(newItem);
  }
  return result.slice(0, n);
}
