import { keymap } from "@codemirror/view";
import { Prec } from "@codemirror/state";
import type { KeyBinding } from "@codemirror/view";

type Actions = {
  openFile: () => void;
  saveFile: () => void;
  showNewFileModal: () => void;
  runCode: () => void;
};

export function getEditorKeybinds(actions: Actions) {
  const bindings: KeyBinding[] = [
    {
      key: "Ctrl-Enter",
      run: () => {
        actions.runCode();
        return true;
      },
    },
    {
      key: "Ctrl-s",
      run: () => {
        actions.saveFile();
        return true;
      },
    },
    {
      key: "Ctrl-o",
      run: () => {
        actions.openFile();
        return true;
      },
    },
    {
      key: "Ctrl-n",
      run: () => {
        actions.showNewFileModal();
        return true;
      },
    },
  ];
  return Prec.highest(keymap.of(bindings));
}

export function registerWindowKeybinds(actions: Actions): () => void {
  function handler(e: KeyboardEvent) {
    if (!e.ctrlKey) return;

    // check if a CodeMirror editor is focused — if so, let it handle
    const cmFocused = document.querySelector(".cm-focused") !== null;

    switch (e.key) {
      case "s":
        e.preventDefault();
        if (!cmFocused) actions.saveFile();
        break;
      case "o":
        e.preventDefault();
        if (!cmFocused) actions.openFile();
        break;
      case "n":
        e.preventDefault();
        if (!cmFocused) actions.showNewFileModal();
        break;
      case "Enter":
        e.preventDefault();
        if (!cmFocused) actions.runCode();
        break;
    }
  }

  window.addEventListener("keydown", handler);
  return () => window.removeEventListener("keydown", handler);
}
