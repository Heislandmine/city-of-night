import React, { useEffect, useState } from "react";
import { GameProgressService } from "../../core/Services/GameProgressService.js";
import { Box, Text, useInput } from "ink";
import { TopStatusBar } from "./TopStatusBar.js";
import { TopActionMenu } from "./TopActionMenu.js";
import { FooterMenu } from "./FooterMenu.js";
import { PurchaseDataService } from "../../core/Services/PurchaseDataService.js";
import { PurchaseService } from "../../core/Services/PurchaseService.js";

export const PurchaseAvailableCharacterListViewActionMenu = ({
  menuItems,
}: {
  menuItems: { callId: number; displayName: string; price: number }[];
}) => {
  return (
    <Box width={"100%"} borderStyle={"classic"} gap={2}>
      {menuItems.map((e) => (
        <Text key={e.displayName}>
          <Text>{`[${e.callId}]${e.displayName}`}</Text>
          <Text> </Text>
          <Text bold>{`${e.price}G`}</Text>
        </Text>
      ))}
    </Box>
  );
};

export const PurchaseAvailableCharacterListView = ({
  gameProgressService,
  purchaseDataService,
  purchaseService,
  navigate,
}: {
  gameProgressService: GameProgressService;
  purchaseDataService: PurchaseDataService;
  purchaseService: PurchaseService;
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
        case "999":
          navigate("Top");
          break;

        default:
          const characters = purchaseDataService.getAllCharacterBaseData();

          const target = characters.find(
            (e) => e.callId.toString() === userInput,
          );

          if (!target) {
            setOutputString("不正な値です");
            return;
          }

          const result = purchaseService.purchaseCharacter(
            target.callId.toString(),
            target.price,
          );

          if (result) {
            setOutputString(`${target?.displayName}を購入しました`);
          } else {
            setOutputString("資金不足です");
          }
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
      <PurchaseAvailableCharacterListViewActionMenu
        menuItems={purchaseDataService.getAllCharacterBaseData()}
      />
      <FooterMenu menuItems={[{ callNumber: 999, displayName: "戻る" }]} />
      <Text>{outputString}</Text>
    </Box>
  );
};
