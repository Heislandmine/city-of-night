import { test, expect, describe } from "vitest";
import { UserInventory } from "../../src/core/AppState/UserInventory";

describe("addCharacterByPurchaseのテスト", () => {
  test("正常系", () => {
    const sut = new UserInventory(2000);

    const result = sut.addCharacterByPurchase("test", 2000);

    expect(result).toBe(true);
    expect(sut.ownedCharacter).toHaveLength(1);
    expect(sut.moneyAmount).toBe(0);
  });

  test("moneyAmountが不足している場合にfalseが返り、ownedCharacterとmoneyAmountが変更されないこと", () => {
    const sut = new UserInventory(100);

    const result = sut.addCharacterByPurchase("test", 2000);

    expect(result).toBe(false);
    expect(sut.ownedCharacter).toHaveLength(0);
    expect(sut.moneyAmount).toBe(100);
  });
});
