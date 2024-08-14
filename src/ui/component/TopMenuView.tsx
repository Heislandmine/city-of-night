import { Box, Text, useInput } from "ink";
import React, { useEffect, useState } from "react";
import { TopStatusBar } from "./TopStatusBar.js";
import { GameProgressService } from "../../core/Services/GameProgressService.js";
import { TopActionMenu } from "./TopActionMenu.js";
import { exit } from "process";

export const TopMenuView = ({
  gameProgressService,
  navigate,
}: {
  gameProgressService: GameProgressService;
  navigate: (viewName: string) => void;
}) => {
  const progress = gameProgressService.currentProgress();
  const [userInput, setUserInput] = useState("");
  const [outputString, setOutputString] = useState("");

  useEffect(() => {
    setOutputString(userInput);
  }, [userInput]);

  useInput((input, key) => {
    if (key.return) {
      switch (userInput) {
        case "300":
          setUserInput("");
          navigate("purchaseCharacter");
          break;

        case "999":
          exit();

        default:
          setOutputString("不正な値です");
          setUserInput("");
          return;
      }
    }

    if (input) {
      setUserInput((prev) => prev + input);
    }
  });

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
          { callNumber: 300, displayName: "奴隷購入" },
          { callNumber: 200, displayName: "保存" },
          { callNumber: 999, displayName: "終了" },
        ]}
      />

      <Text>{outputString}</Text>
    </Box>
  );
};
