import React from "react";
import { Box, render, Spacer, Text } from "ink";
import { TopActionMenu } from "./component/TopActionMenu.js";
import { TopStatusBar } from "./component/TopStatusBar.js";
import { GameProgressService } from "../core/Services/GameProgressService.js";

const App = ({
  gameProgressService,
}: {
  gameProgressService: GameProgressService;
}) => {
  const progress = gameProgressService.currentProgress();

  return (
    <Box width={"100%"} flexDirection="column">
      <TopStatusBar
        date={progress.date}
        phase={progress.phase}
        daysPassed={progress.daysPassed}
        leftDays={progress.lefDays}
      />
      <TopActionMenu
        menuItems={[
          { callNumber: 100, displayName: "調教" },
          { callNumber: 200, displayName: "保存" },
        ]}
      />
    </Box>
  );
};

export const startUI = (gameProgressService: GameProgressService) => {
  render(<App gameProgressService={gameProgressService}></App>);
};
