import React, { useEffect, useState } from "react";
import { GameProgressService } from "../../core/Services/GameProgressService.js";
import { Box, Text, useInput } from "ink";
import { TopStatusBar } from "./TopStatusBar.js";
import { TopActionMenu } from "./TopActionMenu.js";

export const PurchaseAvailableCharacterListView = ({
  gameProgressService,
  navigate,
}: {
  gameProgressService: GameProgressService;
  navigate: (viewName: string) => void;
}) => {
  const [userInput, setUserInput] = useState("");
  const [outputString, setOutputString] = useState("");
  const progress = gameProgressService.currentProgress();

  useEffect(() => {
    setOutputString(userInput);
  }, [userInput]);

  useInput((input, key) => {
    if (input) {
      setUserInput((prev) => prev + input);
    }

    if (key.return) {
      switch (userInput) {
        case "200":
          navigate("Top");
          break;

        default:
          setOutputString("不正な値です");
      }
      setUserInput("");
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
          { callNumber: 1, displayName: "デモ子" },
          { callNumber: 200, displayName: "戻る" },
        ]}
      />
      <Text>{outputString}</Text>
    </Box>
  );
};
