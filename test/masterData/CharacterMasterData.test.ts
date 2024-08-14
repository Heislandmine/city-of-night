import { join } from "path";
import { test, expect } from "vitest";
import { CharacterMasterData } from "../../src/core/MasterData/CharacterMasterData.js";

test("コンストラクタで指定したパスのパターンからキャラクターシートを読み込めること", () => {
  const pattern = join(process.cwd(), "test", "Mods", "core", "character");
  const sut = new CharacterMasterData(pattern);

  const result = sut.getPurchaseData();

  expect(result).toHaveLength(1);
});
