// https://www.codewars.com/kata/525f50e3b73515a6db000b83/
export function createPhoneNumber(numbers: number[]): string {
  return [
    `(${numbers.slice(0, 3).join('')})`,
    ' ',
    numbers.slice(3, 6).join(''),
    '-',
    numbers.slice(6).join(''),
  ].join('');
}
