import type { FC } from 'react';

type SvgViewerProps = {
  svgString: string;
  score: number;
  sqdiff: number;
};

const SvgViewer: FC<SvgViewerProps> = ({ svgString, score, sqdiff }) => {
  return (
    <>
      <div>score={score}</div>
      <div>square diff={sqdiff}</div>
      <div
        dangerouslySetInnerHTML={{
          __html: svgString,
        }}
      />
    </>
  );
};

export default SvgViewer;
