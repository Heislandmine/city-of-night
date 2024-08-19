import { describe, expect, test } from "vitest";
import { PurchaseService } from "../../src/core/Services/PurchaseService";

describe("purchaseCharacterのテスト", () => {
  test("正常系のテスト", () => {
    const inventory = {
      moneyAmount: 2000,
      ownedCharacter: [],
    };
    const sut = new PurchaseService(inventory);

    const result = sut.purchaseCharacter("test:character:demo-ko", 2000);

    expect(result).toBe(true);
    expect(inventory.moneyAmount).toBe(0);
    expect(inventory.ownedCharacter).toHaveLength(1);
  });

  test("資金が不足している場合にfalseを返しinventory.を更新しないこと", () => {
    const inventory = {
      moneyAmount: 100,
      ownedCharacter: [],
    };
    const sut = new PurchaseService(inventory);

    const result = sut.purchaseCharacter("test:character:demo-ko", 2000);

    expect(result).toBe(false);
    expect(inventory.moneyAmount).toBe(100);
    expect(inventory.ownedCharacter).toHaveLength(0);
  });
});
