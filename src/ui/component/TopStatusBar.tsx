import React from "react";
import { Box, Spacer, Text } from "ink";

export const TopStatusBar = () => {
  return (
    <Box borderStyle={"single"} width={"100%"}>
      <Text>2124/4/1</Text>
      <Spacer />
      <Text>(昼)</Text>
      <Spacer />
      <Text>10日目</Text>
      <Spacer />
      <Text>残り200日</Text>
    </Box>
  );
};
