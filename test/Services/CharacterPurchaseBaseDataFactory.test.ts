import { describe, expect, test } from "vitest";
import {
  CharacterPurchaseBaseDataFactory,
  CharacterSheetDataNotFound,
} from "../../src/core/Services/CharacterPurchaseBaseDataFactory";

describe("createの実装", () => {
  test("正常系", () => {
    const sut = new CharacterPurchaseBaseDataFactory();
    const expected = [
      {
        id: "testId",
        callId: 100,
        displayName: "test",
        price: 200,
      },
    ];
    const characterData = {
      id: "testId",
      displayName: "test",
      price: 200,
    };
    const callIdTableItem = {
      callId: 100,
      objectId: "testId",
    };

    const result = sut.create([characterData], [callIdTableItem]);

    expect(result).toEqual(expected);
  });

  test("callIdTableItemに指定されたobjectIdを持つキャラクターがいない場合例外を投げること", () => {
    const sut = new CharacterPurchaseBaseDataFactory();

    const characterData = {
      id: "testId2",
      displayName: "test",
      price: 200,
    };
    const callIdTableItem = {
      callId: 100,
      objectId: "testId",
    };

    expect(() => sut.create([characterData], [callIdTableItem])).toThrow(
      CharacterSheetDataNotFound,
    );
  });
});
