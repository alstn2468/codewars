// https://www.codewars.com/kata/515bb423de843ea99400000a/
export class PaginationHelper<T> {
  collection: T[];
  itemsPerPage: number = 0;

  constructor(collection: T[], itemsPerPage: number) {
    this.collection = collection;
    this.itemsPerPage = itemsPerPage;
  }
  itemCount() {
    return this.collection.length;
  }
  pageCount() {
    return Math.ceil(this.itemCount() / this.itemsPerPage);
  }
  pageItemCount(pageIndex: number) {
    if (pageIndex > this.pageCount() - 1 || pageIndex < 0) {
      return -1;
    }
    return this.collection.slice(
      pageIndex * this.itemsPerPage,
      (pageIndex + 1) * this.itemsPerPage,
    ).length;
  }
  pageIndex(itemIndex: number) {
    if (itemIndex > this.itemCount() - 1 || itemIndex < 0) {
      return -1;
    }
    return Math.floor(itemIndex / this.itemsPerPage);
  }
}
