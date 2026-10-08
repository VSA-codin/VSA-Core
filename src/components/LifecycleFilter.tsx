import { useEffect, useId, useRef, useState, type KeyboardEvent } from "react";
import { moduleLifecycles, type ModuleLifecycle } from "../services/api";

export type LifecycleSelection = ModuleLifecycle | "all";
export const lifecycleLabels: Record<LifecycleSelection, string> = {
  all: "All lifecycles", planned: "Planned", available: "Available", installed: "Installed", enabled: "Enabled",
};
const choices: LifecycleSelection[] = ["all", ...moduleLifecycles];

export function LifecycleFilter({ value, onChange }: {
  value: LifecycleSelection; onChange: (value: LifecycleSelection) => void;
}) {
  const id = useId();
  const root = useRef<HTMLDivElement>(null);
  const button = useRef<HTMLButtonElement>(null);
  const list = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);
  const [active, setActive] = useState(0);

  useEffect(() => {
    if (!open) return;
    list.current?.focus();
    function outside(event: PointerEvent) {
      if (event.target instanceof Node && !root.current?.contains(event.target)) setOpen(false);
    }
    document.addEventListener("pointerdown", outside);
    return () => document.removeEventListener("pointerdown", outside);
  }, [open]);

  function show(index = choices.indexOf(value)) { setActive(index); setOpen(true); }
  function close() { setOpen(false); button.current?.focus(); }
  function choose(index: number) {
    const choice = choices[index];
    if (choice !== undefined) onChange(choice);
    close();
  }
  function navigate(event: KeyboardEvent<HTMLDivElement>) {
    switch (event.key) {
      case "ArrowDown": event.preventDefault(); setActive(index => (index + 1) % choices.length); break;
      case "ArrowUp": event.preventDefault(); setActive(index => (index + choices.length - 1) % choices.length); break;
      case "Home": event.preventDefault(); setActive(0); break;
      case "End": event.preventDefault(); setActive(choices.length - 1); break;
      case "Enter": case " ": event.preventDefault(); choose(active); break;
      case "Escape": event.preventDefault(); event.stopPropagation(); close(); break;
      // Let the browser move focus; the blur handler dismisses the popup.
      case "Tab": break;
      default: {
        if (event.key.length !== 1 || event.ctrlKey || event.metaKey || event.altKey) return;
        const index = choices.findIndex(choice => lifecycleLabels[choice].toLowerCase().startsWith(event.key.toLowerCase()));
        if (index !== -1) { event.preventDefault(); setActive(index); }
      }
    }
  }

  return (
    <div className="lifecycle-filter" ref={root} onBlur={event => {
      if (!event.currentTarget.contains(event.relatedTarget)) setOpen(false);
    }}>
      <span id={`${id}-label`} className="filter-label">Lifecycle</span>
      <button ref={button} type="button" className="select-trigger" aria-haspopup="listbox"
        aria-expanded={open} aria-controls={open ? `${id}-list` : undefined}
        aria-labelledby={`${id}-label ${id}-value`}
        onClick={() => open ? close() : show()}
        onKeyDown={event => {
          if (event.key === "ArrowDown" || event.key === "ArrowUp") {
            event.preventDefault(); show(event.key === "ArrowUp" ? choices.length - 1 : choices.indexOf(value));
          }
        }}>
        <span id={`${id}-value`}>{lifecycleLabels[value]}</span><span aria-hidden="true">⌄</span>
      </button>
      {open && <div ref={list} id={`${id}-list`} className="select-list" role="listbox" tabIndex={0}
        aria-labelledby={`${id}-label`} aria-activedescendant={`${id}-option-${active}`} onKeyDown={navigate}
        onBlur={() => setOpen(false)}>
        {choices.map((choice, index) => <div key={choice} id={`${id}-option-${index}`} role="option"
          aria-selected={choice === value} className={`select-option ${active === index ? "highlighted" : ""}`}
          onPointerMove={() => setActive(index)} onMouseDown={event => event.preventDefault()} onClick={() => choose(index)}>
          <span>{lifecycleLabels[choice]}</span><span aria-hidden="true">{choice === value ? "✓" : ""}</span>
        </div>)}
      </div>}
    </div>
  );
}
