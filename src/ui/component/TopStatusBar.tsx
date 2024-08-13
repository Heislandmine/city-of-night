import React from "react";
import { Box, Spacer, Text } from "ink";

export const TopStatusBar = ({
  date,
  phase,
  leftDays,
  daysPassed,
}: {
  date: string;
  phase: "昼" | "夜";
  leftDays: number;
  daysPassed: number;
}) => {
  return (
    <Box borderStyle={"single"} width={"100%"}>
      <Text>{date}</Text>
      <Spacer />
      <Text>{phase}</Text>
      <Spacer />
      <Text>{`${daysPassed}日目`}</Text>
      <Spacer />
      <Text>{`残り${leftDays}日`}</Text>
    </Box>
  );
};
