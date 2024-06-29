import type { FC } from 'react';
import { useEffect, useState } from 'react';
import { gen, get_max_turn as getMaxTurn, vis } from '../../public/wasm/rust';
import type { VisualizerResult, VisualizerSettingInfo } from '../types';
import Description from './Description';
import FileUploader from './FileUploader';
import InputOutput from './InputOutput';
import SaveButtons from './SaveButtons';
import SvgViewer from './SvgViewer';
import TurnSlider from './TurnSlider';

const AHCLikeVisualizer: FC = () => {
  const [visualizerSettingInfo, setVisualizerSettingInfo] =
    useState<VisualizerSettingInfo>({
      input: '',
      output: '',
      seed: 0,
      turn: 0,
      maxTurn: 0,
    });

  const [visualizerResult, setVisualizerResult] = useState<VisualizerResult>({
    svgString: '',
    score: 0,
    pos: '',
    vel: '',
  });

  useEffect(() => {
    const inputText = gen(visualizerSettingInfo.seed);
    setVisualizerSettingInfo((prev) => ({ ...prev, input: inputText }));
  }, [visualizerSettingInfo.seed]);

  useEffect(() => {
    try {
      const maxTurn = getMaxTurn(
        visualizerSettingInfo.input,
        visualizerSettingInfo.output
      );
      setVisualizerSettingInfo((prev) => ({
        ...prev,
        maxTurn,
        turn: 0,
      }));
    } catch (e) {
      // outputが不正な場合には計算ができない。そのときにはmaxTurnを0にする
      setVisualizerSettingInfo((prev) => ({
        ...prev,
        maxTurn: 0,
        turn: 0,
      }));
    }
  }, [
    visualizerSettingInfo.output,
    visualizerSettingInfo.input,
    setVisualizerSettingInfo,
  ]);

  useEffect(() => {
    try {
      const ret = vis(
        visualizerSettingInfo.input,
        visualizerSettingInfo.output,
        visualizerSettingInfo.turn
      );
      console.log(ret);
      setVisualizerResult({
        svgString: ret.svg,
        score: Number(ret.score),
        pos: ret.pos,
        vel: ret.vel,
      });
    } catch (e) {
      // visが失敗した場合にはエラーを出力する
      console.log(e);
      setVisualizerResult({
        svgString: 'invalid input or output',
        score: 0,
        pos: '',
        vel: '',
      });
    }
  }, [
    visualizerSettingInfo.turn,
    visualizerSettingInfo.input,
    visualizerSettingInfo.output,
  ]);

  return (
    <>
      <Description />
      <hr />
      <FileUploader setVisualizerSettingInfo={setVisualizerSettingInfo} />
      <InputOutput
        visualizerSettingInfo={visualizerSettingInfo}
        setVisualizerSettingInfo={setVisualizerSettingInfo}
      />
      <SaveButtons visualizerSettingInfo={visualizerSettingInfo} />
      <TurnSlider
        visualizerSettingInfo={visualizerSettingInfo}
        setVisualizerSettingInfo={setVisualizerSettingInfo}
      />
      <hr />
      <SvgViewer
        svgString={visualizerResult.svgString}
        score={visualizerResult.score}
        pos={visualizerResult.pos}
        vel={visualizerResult.vel}
      ></SvgViewer>
    </>
  );
};

export default AHCLikeVisualizer;
