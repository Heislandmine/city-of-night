import { GameProgress } from "../GameProgress.js";

export class GameProgressService {
  currentProgress(): GameProgress {
    return {
      date: "2124/8/5",
      phase: "昼",
      daysPassed: 10,
      lefDays: 200,
    };
  }
}
