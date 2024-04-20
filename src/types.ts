export type VisualizerSettingInfo = {
  input: string;
  output: string;
  seed: number;
  problem: string;
  turn: number;
  maxTurn: number;
};

export type VisualizerResult = {
  svgString: string;
  score: number;
};
