import { join } from "path";
import { expect, test } from "vitest";
import { PurchaseCallIdTable } from "../../src/core/Services/PurchaseCallIdTable";

test("コンストラクタに指定した設定ファイルを読み込むこと", () => {
  const testPath = join(
    process.cwd(),
    "test",
    "Mods",
    "core",
    "CallIdTables",
    "PurchaseCallIdTable.json",
  );
  const sut = new PurchaseCallIdTable(testPath);

  const result = sut.get();

  expect(result).toHaveLength(1);
});
