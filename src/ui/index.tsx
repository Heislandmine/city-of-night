import React from "react";
import { Box, render, Text } from "ink";

const App = () => {
  return (
    <Box borderStyle={"single"}>
      <Text>hello</Text>
    </Box>
  );
};

export const startUI = () => {
  render(<App></App>);
};
