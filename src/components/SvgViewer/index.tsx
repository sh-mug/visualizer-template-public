import type { FC } from 'react';

type SvgViewerProps = {
  svgString: string;
  score: number;
  pos: string;
  vel: string;
};

const SvgViewer: FC<SvgViewerProps> = ({ svgString, score, pos, vel }) => {
  return (
    <>
      <div>score={score}</div>
      <div>position={pos}</div>
      <div>velocity={vel}</div>
      <div
        dangerouslySetInnerHTML={{
          __html: svgString,
        }}
      />
    </>
  );
};

export default SvgViewer;
