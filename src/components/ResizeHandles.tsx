import { getCurrentWindow } from "@tauri-apps/api/window";

type ResizeDirection =
  | "North"
  | "South"
  | "East"
  | "West"
  | "NorthEast"
  | "NorthWest"
  | "SouthEast"
  | "SouthWest";

const DIRECTIONS: { direction: ResizeDirection; className: string }[] = [
  { direction: "North", className: "resize-handle resize-handle-n" },
  { direction: "South", className: "resize-handle resize-handle-s" },
  { direction: "East", className: "resize-handle resize-handle-e" },
  { direction: "West", className: "resize-handle resize-handle-w" },
  { direction: "NorthEast", className: "resize-handle resize-handle-ne" },
  { direction: "NorthWest", className: "resize-handle resize-handle-nw" },
  { direction: "SouthEast", className: "resize-handle resize-handle-se" },
  { direction: "SouthWest", className: "resize-handle resize-handle-sw" },
];

function ResizeHandles() {
  const appWindow = getCurrentWindow();

  const handleMouseDown = (direction: ResizeDirection) => (e: React.MouseEvent) => {
    e.preventDefault();
    appWindow.startResizeDragging(direction);
  };

  return (
    <>
      {DIRECTIONS.map(({ direction, className }) => (
        <div
          key={direction}
          className={className}
          onMouseDown={handleMouseDown(direction)}
        />
      ))}
    </>
  );
}

export default ResizeHandles;
