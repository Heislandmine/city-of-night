import { expect, test } from "vitest";
import {
  CallIdTableItem,
  CharacterSheet,
  ICallIdTable,
  ICharacterSheets,
  PurchaseDataService,
} from "../../src/core/Services/PurchaseDataService";
import { CharacterPurchaseBaseDataFactory } from "../../src/core/Services/CharacterPurchaseBaseDataFactory";

class TestPurchaseCallIdTable implements ICallIdTable {
  constructor(private readonly data: CallIdTableItem[]) {}

  get(): CallIdTableItem[] {
    return this.data;
  }
}

class TestCharacterSheets implements ICharacterSheets {
  constructor(private _characterSheets: CharacterSheet[]) {}

  getAll(): CharacterSheet[] {
    return this._characterSheets;
  }
}

test("getAllCharacters", () => {
  const purchaseCallIdTable = new TestPurchaseCallIdTable([
    { callId: 1, objectId: "test:character:demo-ko" },
  ]);
  const characterSheets = new TestCharacterSheets([
    {
      id: "test:character:demo-ko",
      displayName: "テスト子",
      price: 2000,
    },
  ]);
  const purchaseBaseDataFactory = new CharacterPurchaseBaseDataFactory();

  const expected = {
    id: "test:character:demo-ko",
    callId: 1,
    displayName: "テスト子",
    price: 2000,
  };
  const sut = new PurchaseDataService(
    characterSheets,
    purchaseCallIdTable,
    purchaseBaseDataFactory,
  );

  const result = sut.getAllCharacterBaseData();

  expect(result).toEqual([expected]);
});
