export const digitalRoot = (n: number): number => {
  const num = n.toString();

  if (num.length < 2) {
    return Number(num);
  }

  return digitalRoot(
    num.split('').reduce((prev, each) => Number(each) + prev, 0),
  );
};
