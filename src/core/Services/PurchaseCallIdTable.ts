import { readFileSync } from "fs";
import { CallIdTableItem, ICallIdTable } from "./PurchaseDataService.js";

export class PurchaseCallIdTable implements ICallIdTable {
  private readonly _table: CallIdTableItem[];
  constructor(filePath: string) {
    this._table = JSON.parse(readFileSync(filePath, { encoding: "utf-8" }));
  }

  get(): CallIdTableItem[] {
    return [...this._table];
  }
}
