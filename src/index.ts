import { join } from "path";
import { GameProgressService } from "./core/Services/GameProgressService.js";
import { PurchaseCallIdTable } from "./core/Services/PurchaseCallIdTable.js";
import { PurchaseDataService } from "./core/Services/PurchaseDataService.js";
import { startUI } from "./ui/index.js";
import { CharacterPurchaseBaseDataFactory } from "./core/Services/CharacterPurchaseBaseDataFactory.js";
import { CharacterMasterData } from "./core/MasterData/CharacterMasterData.js";
import { PurchaseService } from "./core/Services/PurchaseService.js";
import { UserInventory } from "./core/AppState/UserInventory.js";

const gameProgressService = new GameProgressService();
const purchaseDataService = new PurchaseDataService(
  new CharacterMasterData(join(process.cwd(), "Mods", "core", "character")),
  new PurchaseCallIdTable(
    join(
      process.cwd(),
      "Mods",
      "core",
      "CallIdTables",
      "PurchaseCallIdTable.json",
    ),
  ),
  new CharacterPurchaseBaseDataFactory(),
);
startUI(
  gameProgressService,
  purchaseDataService,
  new PurchaseService(new UserInventory(20000)),
);
