const bigSort = (array, reversed = false) => {
  array.sort((a, b) => {
    if (a > b) {
      return reversed ? -1 : 1;
    } else if (a < b) {
      return reversed ? 1 : -1;
    } else {
      return 0;
    }
  });
};
