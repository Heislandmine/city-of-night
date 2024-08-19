import { describe, expect, test } from "vitest";
import { PurchaseService } from "../../src/core/Services/PurchaseService";
import { UserInventory } from "../../src/core/AppState/UserInventory";

describe("purchaseCharacterのテスト", () => {
  test("正常系のテスト", () => {
    const inventory = new UserInventory(2000);
    const sut = new PurchaseService(inventory);

    const result = sut.purchaseCharacter("test:character:demo-ko", 2000);

    expect(result).toBe(true);
  });

  test("資金が不足している場合にfalseを返しinventory.を更新しないこと", () => {
    const inventory = new UserInventory(100);
    const sut = new PurchaseService(inventory);

    const result = sut.purchaseCharacter("test:character:demo-ko", 2000);

    expect(result).toBe(false);
  });
});
