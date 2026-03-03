import { Excalidraw } from "@excalidraw/excalidraw";
import type { ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import "@excalidraw/excalidraw/index.css";

interface ExcalidrawWrapperProps {
  onApiReady?: (api: ExcalidrawImperativeAPI) => void;
}

export function ExcalidrawWrapper({ onApiReady }: ExcalidrawWrapperProps) {
  return <Excalidraw excalidrawAPI={(api) => onApiReady?.(api)} />;
}

export type { ExcalidrawImperativeAPI };
