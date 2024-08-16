import { expect, test } from "vitest";
import {
  CallIdTableItem,
  ICallIdTable,
  PurchaseDataService,
} from "../../src/core/Services/PurchaseDataService";
import { CharacterPurchaseBaseDataFactory } from "../../src/core/Services/CharacterPurchaseBaseDataFactory";

class TestPurchaseCallIdTable implements ICallIdTable {
  constructor(private readonly data: CallIdTableItem[]) {}

  get(): CallIdTableItem[] {
    return this.data;
  }
}

test("getAllCharacters", () => {
  const purchaseCallIdTable = new TestPurchaseCallIdTable([
    { callId: 1, objectId: "test:character:demo-ko" },
  ]);
  const purchaseBaseDataFactory = new CharacterPurchaseBaseDataFactory();

  const expected = {
    id: "test:character:demo-ko",
    callId: 1,
    displayName: "テスト子",
    price: 2000,
  };
  const sut = new PurchaseDataService(
    purchaseCallIdTable,
    purchaseBaseDataFactory,
  );

  const result = sut.getAllCharacterBaseData();

  expect(result).toEqual([expected]);
});
