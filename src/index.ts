import { GameProgressService } from "./core/Services/GameProgressService.js";
import { startUI } from "./ui/index.js";

const gameProgressService = new GameProgressService();
startUI(gameProgressService);
